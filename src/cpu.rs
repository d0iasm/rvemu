//! The cpu module contains the privileged mode, registers, and CPU.

use std::cmp;
use std::cmp::PartialEq;
use std::collections::BTreeMap;
use std::fmt;
use std::num::FpCategory;

use crate::{
    bus::{Bus, DRAM_BASE},
    csr::*,
    devices::{
        plic::PLIC_SCLAIM,
        uart::UART_IRQ,
        virtio::{Virtio, VIRTIO_IRQ},
    },
    dram::DRAM_SIZE,
    exception::Exception,
    interrupt::Interrupt,
};

/// The stack pointer.
const SP: u64 = 2;

/// The number of registers.
const REGISTERS_COUNT: usize = 32;

/// The page size (4 KiB) for the virtual memory system.
const PAGE_SIZE: u64 = 4096;

/// 8 bits. 1 byte.
pub const BYTE: u8 = 8;
/// 16 bits. 2 bytes.
pub const HALFWORD: u8 = 16;
/// 32 bits. 4 bytes.
pub const WORD: u8 = 32;
/// 64 bits. 8 bytes.
pub const DOUBLEWORD: u8 = 64;

macro_rules! inst_count {
    ($cpu:ident, $inst_name:expr) => {
        if $cpu.is_count {
            *$cpu.inst_counter.entry($inst_name.to_string()).or_insert(0) += 1;
        }
    };
}

/// Access type that is used in the virtual address translation process. It decides which exception
/// should raises (InstructionPageFault, LoadPageFault or StoreAMOPageFault).
#[derive(Debug, PartialEq, PartialOrd)]
pub enum AccessType {
    /// Raises the exception InstructionPageFault. It is used for an instruction fetch.
    Instruction,
    /// Raises the exception LoadPageFault.
    Load,
    /// Raises the exception StoreAMOPageFault.
    Store,
}

/// The privileged mode.
#[derive(Debug, PartialEq, PartialOrd, Eq, Copy, Clone)]
pub enum Mode {
    User = 0b00,
    Supervisor = 0b01,
    Machine = 0b11,
    Debug,
}

/// The integer registers.
#[derive(Debug)]
pub struct XRegisters {
    xregs: [u64; REGISTERS_COUNT],
}

impl XRegisters {
    /// Create a new `XRegisters` object.
    pub fn new() -> Self {
        let mut xregs = [0; REGISTERS_COUNT];
        // The stack pointer is set in the default maximum mamory size + the start address of dram.
        xregs[SP as usize] = DRAM_BASE + DRAM_SIZE;
        // From riscv-pk:
        // https://github.com/riscv/riscv-pk/blob/master/machine/mentry.S#L233-L235
        //   save a0 and a1; arguments from previous boot loader stage:
        //   // li x10, 0
        //   // li x11, 0
        //
        // void init_first_hart(uintptr_t hartid, uintptr_t dtb)
        //   x10 (a0): hartid
        //   x11 (a1): pointer to dtb
        //
        // So, we need to set registers register to the state as they are when a bootloader finished.
        xregs[10] = 0;
        xregs[11] = 0x1020;
        Self { xregs }
    }

    /// Read the value from a register.
    pub fn read(&self, index: u64) -> u64 {
        self.xregs[index as usize]
    }

    /// Write the value to a register.
    pub fn write(&mut self, index: u64, value: u64) {
        // Register x0 is hardwired with all bits equal to 0.
        if index != 0 {
            self.xregs[index as usize] = value;
        }
    }
}

impl fmt::Display for XRegisters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let abi = [
            "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ",
            " a1 ", " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ",
            " s6 ", " s7 ", " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
        ];
        let mut output = String::from("");
        for i in (0..REGISTERS_COUNT).step_by(4) {
            output = format!(
                "{}\n{}",
                output,
                format!(
                    "x{:02}({})={:>#18x} x{:02}({})={:>#18x} x{:02}({})={:>#18x} x{:02}({})={:>#18x}",
                    i,
                    abi[i],
                    self.read(i as u64),
                    i + 1,
                    abi[i + 1],
                    self.read(i as u64 + 1),
                    i + 2,
                    abi[i + 2],
                    self.read(i as u64 + 2),
                    i + 3,
                    abi[i + 3],
                    self.read(i as u64 + 3),
                )
            );
        }
        write!(f, "{}", output)
    }
}

/// The floating-point registers.
#[derive(Debug)]
pub struct FRegisters {
    fregs: [f64; REGISTERS_COUNT],
}

impl FRegisters {
    /// Create a new `FRegisters` object.
    pub fn new() -> Self {
        Self {
            fregs: [0.0; REGISTERS_COUNT],
        }
    }

    /// Read the value from a regsiter.
    pub fn read(&self, index: u64) -> f64 {
        self.fregs[index as usize]
    }

    /// Write the value to a regsiter.
    pub fn write(&mut self, index: u64, value: f64) {
        self.fregs[index as usize] = value;
    }
}

impl fmt::Display for FRegisters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let abi = [
            // ft0-7: FP temporaries
            " ft0", " ft1", " ft2", " ft3", " ft4", " ft5", " ft6", " ft7",
            // fs0-1: FP saved registers
            " fs0", " fs1", // fa0-1: FP arguments/return values
            " fa0", " fa1", // fa2–7: FP arguments
            " fa2", " fa3", " fa4", " fa5", " fa6", " fa7",
            // fs2–11: FP saved registers
            " fs2", " fs3", " fs4", " fs5", " fs6", " fs7", " fs8", " fs9", "fs10", "fs11",
            // ft8–11: FP temporaries
            " ft8", " ft9", "ft10", "ft11",
        ];
        let mut output = String::from("");
        for i in (0..REGISTERS_COUNT).step_by(4) {
            output = format!(
                "{}\n{}",
                output,
                format!(
                    "f{:02}({})={:>width$.prec$} f{:02}({})={:>width$.prec$} f{:02}({})={:>width$.prec$} f{:02}({})={:>width$.prec$}",
                    i,
                    abi[i],
                    self.read(i as u64),
                    i + 1,
                    abi[i + 1],
                    self.read(i as u64 + 1),
                    i + 2,
                    abi[i + 2],
                    self.read(i  as u64+ 2),
                    i + 3,
                    abi[i + 3],
                    self.read(i as u64 + 3),
                    width=18,
                    prec=8,
                )
            );
        }
        // Remove the first new line.
        output.remove(0);
        write!(f, "{}", output)
    }
}

/// The CPU to contain registers, a program coutner, status, and a privileged mode.
pub struct Cpu {
    /// 64-bit integer registers.
    pub xregs: XRegisters,
    /// 64-bit floating-point registers.
    pub fregs: FRegisters,
    /// Program coutner.
    pub pc: u64,
    /// Control and status registers (CSR).
    pub state: State,
    /// Privilege level.
    pub mode: Mode,
    /// Previous privilege level.
    pub prev_mode: Mode,
    /// System bus.
    pub bus: Bus,
    /// SV39 paging flag.
    enable_paging: bool,
    /// Physical page number (PPN) × PAGE_SIZE (4096).
    page_table: u64,
    /// A set of bytes that subsumes the bytes in the addressed word used in
    /// load-reserved/store-conditional instructions.
    reservation_set: Vec<u64>,
    /// Idle state. True when WFI is called, and becomes false when an interrupt happens.
    pub idle: bool,
    /// Counter of each instructions for debug.
    pub inst_counter: BTreeMap<String, u64>,
    /// The count flag. Count the number of each instruction executed.
    pub is_count: bool,
}

impl Cpu {
    /// Create a new `Cpu` object.
    pub fn new() -> Cpu {
        Cpu {
            xregs: XRegisters::new(),
            fregs: FRegisters::new(),
            pc: 0,
            state: State::new(),
            mode: Mode::Machine,
            prev_mode: Mode::Machine,
            bus: Bus::new(),
            enable_paging: false,
            page_table: 0,
            reservation_set: Vec::new(),
            idle: false,
            inst_counter: BTreeMap::new(),
            is_count: false,
        }
    }

    /// Reset CPU states.
    pub fn reset(&mut self) {
        self.pc = 0;
        self.mode = Mode::Machine;
        self.prev_mode = Mode::Machine;
        self.state.reset();
        for i in 0..REGISTERS_COUNT {
            self.xregs.write(i as u64, 0);
            self.fregs.write(i as u64, 0.0);
        }
    }

    /// Check interrupt flags for all devices that can interrupt.
    pub fn check_pending_interrupt(&mut self) -> Option<Interrupt> {
        // global interrupt: PLIC (Platform Local Interrupt Controller) dispatches global
        //                   interrupts to multiple harts.
        // local interrupt: CLINT (Core Local Interrupter) dispatches local interrupts to a hart
        //                  which directly connected to CLINT.

        // 3.1.6.1 Privilege and Global Interrupt-Enable Stack in mstatus register
        // "When a hart is executing in privilege mode x, interrupts are globally enabled when
        // xIE=1 and globally disabled when xIE=0."
        match self.mode {
            Mode::Machine => {
                // Check if the MIE bit is enabled.
                if (self.state.read(MSTATUS) >> 3) & 1 == 0 {
                    return None;
                }
            }
            Mode::Supervisor => {
                // Check if the SIE bit is enabled.
                if (self.state.read(SSTATUS) >> 1) & 1 == 0 {
                    return None;
                }
            }
            _ => {}
        }

        // TODO: Take interrupts based on priorities.

        // Check external interrupt for uart and virtio.
        let irq;
        if self.bus.uart.is_interrupting() {
            irq = UART_IRQ;
        } else if self.bus.virtio.is_interrupting() {
            // An interrupt is raised after a disk access is done.
            Virtio::disk_access(self).expect("failed to access the disk");
            irq = VIRTIO_IRQ;
        } else {
            irq = 0;
        }

        if irq != 0 {
            // TODO: assume that hart is 0
            // TODO: write a value to MCLAIM if the mode is machine
            self.bus
                .write(PLIC_SCLAIM, irq, WORD)
                .expect("failed to write an IRQ to the PLIC_SCLAIM");
            self.state.write(MIP, self.state.read(MIP) | SEIP_BIT);
        }

        // 3.1.9 Machine Interrupt Registers (mip and mie)
        // "An interrupt i will be taken if bit i is set in both mip and mie, and if interrupts are
        // globally enabled. By default, M-mode interrupts are globally enabled if the hart’s
        // current privilege mode is less than M, or if the current privilege mode is M and the MIE
        // bit in the mstatus register is set. If bit i in mideleg is set, however, interrupts are
        // considered to be globally enabled if the hart’s current privilege mode equals the
        // delegated privilege mode (S or U) and that mode’s interrupt enable bit (SIE or UIE in
        // mstatus) is set, or if the current privilege mode is less than the delegated privilege
        // mode."
        let pending = self.state.read(MIE) & self.state.read(MIP);

        if (pending & MEIP_BIT) != 0 {
            //println!("meip: check_pending_interrupt!");
            self.state.write(MIP, self.state.read(MIP) & !MEIP_BIT);
            return Some(Interrupt::MachineExternalInterrupt);
        }
        if (pending & MSIP_BIT) != 0 {
            //println!("msip: check_pending_interrupt!");
            self.state.write(MIP, self.state.read(MIP) & !MSIP_BIT);
            return Some(Interrupt::MachineSoftwareInterrupt);
        }
        if (pending & MTIP_BIT) != 0 {
            //println!("mtip: check_pending_interrupt!");
            self.state.write(MIP, self.state.read(MIP) & !MTIP_BIT);
            return Some(Interrupt::MachineTimerInterrupt);
        }
        if (pending & SEIP_BIT) != 0 {
            //println!("seip: check_pending_interrupt!");
            self.state.write(MIP, self.state.read(MIP) & !SEIP_BIT);
            return Some(Interrupt::SupervisorExternalInterrupt);
        }
        if (pending & SSIP_BIT) != 0 {
            //println!("ssip: check_pending_interrupt!");
            self.state.write(MIP, self.state.read(MIP) & !SSIP_BIT);
            return Some(Interrupt::SupervisorSoftwareInterrupt);
        }
        if (pending & STIP_BIT) != 0 {
            //println!("stip: check_pending_interrupt!");
            self.state.write(MIP, self.state.read(MIP) & !STIP_BIT);
            return Some(Interrupt::SupervisorTimerInterrupt);
        }

        return None;
    }

    /// Update the physical page number (PPN) and the addressing mode.
    fn update_paging(&mut self) {
        // Read the physical page number (PPN) of the root page table, i.e., its
        // supervisor physical address divided by 4 KiB.
        self.page_table = self.state.read_bits(SATP, ..44) * PAGE_SIZE;

        // Read the MODE field, which selects the current address-translation scheme.
        let mode = self.state.read_bits(SATP, 60..);

        // Enable the SV39 paging if the value of the mode field is 8.
        if mode == 8 {
            self.enable_paging = true;
        } else {
            self.enable_paging = false;
        }
    }

    /// Translate a virtual address to a physical address for the paged virtual-memory system.
    fn translate(&mut self, addr: u64, access_type: AccessType) -> Result<u64, Exception> {
        if !self.enable_paging || self.mode == Mode::Machine {
            return Ok(addr);
        }

        // 4.3.2 Virtual Address Translation Process
        // (The RISC-V Instruction Set Manual Volume II-Privileged Architecture_20190608)
        // A virtual address va is translated into a physical address pa as follows:
        let levels = 3;
        let vpn = [
            (addr >> 12) & 0x1ff,
            (addr >> 21) & 0x1ff,
            (addr >> 30) & 0x1ff,
        ];

        // 1. Let a be satp.ppn × PAGESIZE, and let i = LEVELS − 1. (For Sv32, PAGESIZE=212
        //    and LEVELS=2.)
        let mut a = self.page_table;
        let mut i: i64 = levels - 1;
        let mut pte;
        loop {
            // 2. Let pte be the value of the PTE at address a+va.vpn[i]×PTESIZE. (For Sv32,
            //    PTESIZE=4.) If accessing pte violates a PMA or PMP check, raise an access
            //    exception corresponding to the original access type.
            pte = self.bus.read(a + vpn[i as usize] * 8, DOUBLEWORD)?;

            // 3. If pte.v = 0, or if pte.r = 0 and pte.w = 1, stop and raise a page-fault
            //    exception corresponding to the original access type.
            let v = pte & 1;
            let r = (pte >> 1) & 1;
            let w = (pte >> 2) & 1;
            let x = (pte >> 3) & 1;
            if v == 0 || (r == 0 && w == 1) {
                match access_type {
                    AccessType::Instruction => return Err(Exception::InstructionPageFault),
                    AccessType::Load => return Err(Exception::LoadPageFault),
                    AccessType::Store => return Err(Exception::StoreAMOPageFault),
                }
            }

            // 4. Otherwise, the PTE is valid. If pte.r = 1 or pte.x = 1, go to step 5.
            //    Otherwise, this PTE is a pointer to the next level of the page table.
            //    Let i = i − 1. If i < 0, stop and raise a page-fault exception
            //    corresponding to the original access type. Otherwise,
            //    let a = pte.ppn × PAGESIZE and go to step 2.
            if r == 1 || x == 1 {
                break;
            }
            i -= 1;
            let ppn = (pte >> 10) & 0x0fff_ffff_ffff;
            a = ppn * PAGE_SIZE;
            if i < 0 {
                match access_type {
                    AccessType::Instruction => return Err(Exception::InstructionPageFault),
                    AccessType::Load => return Err(Exception::LoadPageFault),
                    AccessType::Store => return Err(Exception::StoreAMOPageFault),
                }
            }
        }
        // TODO: implement step 5
        // 5. A leaf PTE has been found. Determine if the requested memory access is
        //    allowed by the pte.r, pte.w, pte.x, and pte.u bits, given the current
        //    privilege mode and the value of the SUM and MXR fields of the mstatus
        //    register. If not, stop and raise a page-fault exception corresponding
        //    to the original access type.

        // 3.1.6.3 Memory Privilege in mstatus Register
        // "The MXR (Make eXecutable Readable) bit modifies the privilege with which loads access
        // virtual memory. When MXR=0, only loads from pages marked readable (R=1 in Figure 4.15)
        // will succeed. When MXR=1, loads from pages marked either readable or executable
        // (R=1 or X=1) will succeed. MXR has no effect when page-based virtual memory is not in
        // effect. MXR is hardwired to 0 if S-mode is not supported."

        // "The SUM (permit Supervisor User Memory access) bit modifies the privilege with which
        // S-mode loads and stores access virtual memory. When SUM=0, S-mode memory accesses to
        // pages that are accessible by U-mode (U=1 in Figure 4.15) will fault. When SUM=1, these
        // accesses are permitted.  SUM has no effect when page-based virtual memory is not in
        // effect. Note that, while SUM is ordinarily ignored when not executing in S-mode, it is
        // in effect when MPRV=1 and MPP=S. SUM is hardwired to 0 if S-mode is not supported."

        // 6. If i > 0 and pte.ppn[i−1:0] != 0, this is a misaligned superpage; stop and
        //    raise a page-fault exception corresponding to the original access type.
        let ppn = [
            (pte >> 10) & 0x1ff,
            (pte >> 19) & 0x1ff,
            (pte >> 28) & 0x03ff_ffff,
        ];
        if i > 0 {
            for j in (0..i).rev() {
                if ppn[j as usize] != 0 {
                    // A misaligned superpage.
                    match access_type {
                        AccessType::Instruction => return Err(Exception::InstructionPageFault),
                        AccessType::Load => return Err(Exception::LoadPageFault),
                        AccessType::Store => return Err(Exception::StoreAMOPageFault),
                    }
                }
            }
        }

        // 7. If pte.a = 0, or if the memory access is a store and pte.d = 0, either raise
        //    a page-fault exception corresponding to the original access type, or:
        //    • Set pte.a to 1 and, if the memory access is a store, also set pte.d to 1.
        //    • If this access violates a PMA or PMP check, raise an access exception
        //    corresponding to the original access type.
        //    • This update and the loading of pte in step 2 must be atomic; in particular,
        //    no intervening store to the PTE may be perceived to have occurred in-between.
        let a = (pte >> 6) & 1;
        let d = (pte >> 7) & 1;
        if a == 0 || (access_type == AccessType::Store && d == 0) {
            // Set pte.a to 1 and, if the memory access is a store, also set pte.d to 1.
            pte = pte
                | (1 << 6)
                | if access_type == AccessType::Store {
                    1 << 7
                } else {
                    0
                };

            // TODO: PMA or PMP check.

            // Update the value of address satp.ppn × PAGESIZE + va.vpn[i] × PTESIZE with new pte
            // value.
            // TODO: If this is enabled, running xv6 fails.
            //self.bus
            //.write64(self.page_table + vpn[i as usize] * 8, pte)?;
        }

        // 8. The translation is successful. The translated physical address is given as
        //    follows:
        //    • pa.pgoff = va.pgoff.
        //    • If i > 0, then this is a superpage translation and pa.ppn[i−1:0] =
        //    va.vpn[i−1:0].
        //    • pa.ppn[LEVELS−1:i] = pte.ppn[LEVELS−1:i].
        let offset = addr & 0xfff;
        match i {
            0 => {
                let ppn = (pte >> 10) & 0x0fff_ffff_ffff;
                Ok((ppn << 12) | offset)
            }
            1 => {
                // Superpage translation. A superpage is a memory page of larger size than an
                // ordinary page (4 KiB). It reduces TLB misses and improves performance.
                Ok((ppn[2] << 30) | (ppn[1] << 21) | (vpn[0] << 12) | offset)
            }
            2 => {
                // Superpage translation. A superpage is a memory page of larger size than an
                // ordinary page (4 KiB). It reduces TLB misses and improves performance.
                Ok((ppn[2] << 30) | (vpn[1] << 21) | (vpn[0] << 12) | offset)
            }
            _ => match access_type {
                AccessType::Instruction => return Err(Exception::InstructionPageFault),
                AccessType::Load => return Err(Exception::LoadPageFault),
                AccessType::Store => return Err(Exception::StoreAMOPageFault),
            },
        }
    }

    /// Read `size`-bit data from the system bus with the translation a virtual address to a physical address
    /// if it is enabled.
    fn read(&mut self, v_addr: u64, size: u8) -> Result<u64, Exception> {
        let p_addr = self.translate(v_addr, AccessType::Load)?;
        match size {
            BYTE => self.bus.read(p_addr, BYTE),
            HALFWORD => self.bus.read(p_addr, HALFWORD),
            WORD => self.bus.read(p_addr, WORD),
            DOUBLEWORD => self.bus.read(p_addr, DOUBLEWORD),
            _ => Err(Exception::LoadAccessFault),
        }
    }

    /// Write `size`-bit data to the system bus with the translation a virtual address to a physical address
    /// if it is enabled.
    fn write(&mut self, v_addr: u64, value: u64, size: u8) -> Result<(), Exception> {
        // "The SC must fail if a write from some other device to the bytes accessed by the LR can
        // be observed to occur between the LR and SC."
        if self.reservation_set.contains(&v_addr) {
            self.reservation_set.retain(|&x| x != v_addr);
        }

        let p_addr = self.translate(v_addr, AccessType::Load)?;
        match size {
            BYTE => self.bus.write(p_addr, value, BYTE),
            HALFWORD => self.bus.write(p_addr, value, HALFWORD),
            WORD => self.bus.write(p_addr, value, WORD),
            DOUBLEWORD => self.bus.write(p_addr, value, DOUBLEWORD),
            _ => Err(Exception::StoreAMOAccessFault),
        }
    }

    /// Fetch the `size`-bit next instruction from the memory at the current program counter.
    pub fn fetch(&mut self, size: u8) -> Result<u64, Exception> {
        let p_pc = self.translate(self.pc, AccessType::Instruction)?;
        // The result of the read method can be `Exception::LoadAccessFault`. In fetch(), an error
        // should be `Exception::InstructionAccessFault`.
        let result = match size {
            HALFWORD => self.bus.read(p_pc, HALFWORD),
            WORD => self.bus.read(p_pc, WORD),
            _ => Err(Exception::InstructionAccessFault),
        };

        match result {
            Ok(value) => Ok(value),
            Err(_) => Err(Exception::InstructionAccessFault),
        }
    }

    /// Execute a cycle on peripheral devices.
    pub fn devices_increment(&mut self) {
        // TODO: mtime in Clint and TIME in CSR should be the same value.
        // Increment the timer register (mtimer) in Clint.
        self.bus.clint.increment(&mut self.state);
        // Increment the value in the TIME and CYCLE registers in CSR.
        self.state.increment_time();
    }

    /// Execute an instruction. Raises an exception if something is wrong, otherwise, returns
    /// the instruction executed in this cycle.
    pub fn execute(&mut self) -> Result<u64, Exception> {
        // WFI is called and pending interrupts don't exist.
        if self.idle {
            return Ok(0);
        }

        // Fetch.
        let inst16 = self.fetch(HALFWORD)?;
        let inst;
        match inst16 & 0b11 {
            0 | 1 | 2 => {
                if inst16 == 0 {
                    // Unimplemented instruction, since all bits are 0.
                    return Err(Exception::IllegalInstruction);
                }
                inst = self.execute_compressed()?
            }
            _ => inst = self.execute_general()?,
        }
        Ok(inst)
    }

    /// Execute a compressed instruction. Raised an exception if something is wrong, otherwise,
    /// returns a fetched instruction. It also increments the program counter by 2 bytes.
    pub fn execute_compressed(&mut self) -> Result<u64, Exception> {
        // 1. Fetch.
        let inst = self.fetch(HALFWORD)?;

        // Add 2 bytes to the program counter.
        self.pc += 2;

        // 2. Decode.
        let opcode = inst & 0x3;
        let funct3 = (inst >> 13) & 0x7;

        // 3. Execute.
        // Compressed instructions have 3-bit field for popular registers, which correspond to
        // registers x8 to x15.
        match opcode {
            0 => {
                // Quadrant 0.
                match funct3 {
                    0x0 => {
                        // c.addi4spn
                        // Expands to addi rd, x2, nzuimm, where rd=rd'+8.
                        inst_count!(self, "c.addi4spn");

                        let rd = ((inst >> 2) & 0x7) + 8;
                        // nzuimm[5:4|9:6|2|3] = inst[12:11|10:7|6|5]
                        let nzuimm = ((inst >> 1) & 0x3c0) // znuimm[9:6]
                            | ((inst >> 7) & 0x30) // znuimm[5:4]
                            | ((inst >> 2) & 0x8) // znuimm[3]
                            | ((inst >> 4) & 0x4); // znuimm[2]
                        if nzuimm == 0 {
                            return Err(Exception::IllegalInstruction);
                        }
                        self.xregs
                            .write(rd, self.xregs.read(2).wrapping_add(nzuimm));
                    }
                    0x1 => {
                        // c.fld
                        // Expands to fld rd, offset(rs1), where rd=rd'+8 and rs1=rs1'+8.
                        inst_count!(self, "c.fld");

                        let rd = ((inst >> 2) & 0x7) + 8;
                        let rs1 = ((inst >> 7) & 0x7) + 8;
                        // offset[5:3|7:6] = isnt[12:10|6:5]
                        let offset = ((inst << 1) & 0xc0) // imm[7:6]
                            | ((inst >> 7) & 0x38); // imm[5:3]
                        let val = f64::from_bits(
                            self.read(self.xregs.read(rs1).wrapping_add(offset), DOUBLEWORD)?,
                        );
                        self.fregs.write(rd, val);
                    }
                    0x2 => {
                        // c.lw
                        // Expands to lw rd, offset(rs1), where rd=rd'+8 and rs1=rs1'+8.
                        inst_count!(self, "c.lw");

                        let rd = ((inst >> 2) & 0x7) + 8;
                        let rs1 = ((inst >> 7) & 0x7) + 8;
                        // offset[5:3|2|6] = isnt[12:10|6|5]
                        let offset = ((inst << 1) & 0x40) // imm[6]
                            | ((inst >> 7) & 0x38) // imm[5:3]
                            | ((inst >> 4) & 0x4); // imm[2]
                        let addr = self.xregs.read(rs1).wrapping_add(offset);
                        let val = self.read(addr, WORD)?;
                        self.xregs.write(rd, val as i32 as i64 as u64);
                    }
                    0x3 => {
                        // c.ld
                        // Expands to ld rd, offset(rs1), where rd=rd'+8 and rs1=rs1'+8.
                        inst_count!(self, "c.ld");

                        let rd = ((inst >> 2) & 0x7) + 8;
                        let rs1 = ((inst >> 7) & 0x7) + 8;
                        // offset[5:3|7:6] = isnt[12:10|6:5]
                        let offset = ((inst << 1) & 0xc0) // imm[7:6]
                            | ((inst >> 7) & 0x38); // imm[5:3]
                        let addr = self.xregs.read(rs1).wrapping_add(offset);
                        let val = self.read(addr, DOUBLEWORD)?;
                        self.xregs.write(rd, val);
                    }
                    0x4 => {
                        // Reserved.
                        panic!("reserved");
                    }
                    0x5 => {
                        // c.fsd
                        // Expands to fsd rs2, offset(rs1), where rs2=rs2'+8 and rs1=rs1'+8.
                        inst_count!(self, "c.fsd");

                        let rs2 = ((inst >> 2) & 0x7) + 8;
                        let rs1 = ((inst >> 7) & 0x7) + 8;
                        // offset[5:3|7:6] = isnt[12:10|6:5]
                        let offset = ((inst << 1) & 0xc0) // imm[7:6]
                            | ((inst >> 7) & 0x38); // imm[5:3]
                        let addr = self.xregs.read(rs1).wrapping_add(offset);
                        self.write(addr, self.fregs.read(rs2).to_bits() as u64, DOUBLEWORD)?;
                    }
                    0x6 => {
                        // c.sw
                        // Expands to sw rs2, offset(rs1), where rs2=rs2'+8 and rs1=rs1'+8.
                        inst_count!(self, "c.sw");

                        let rs2 = ((inst >> 2) & 0x7) + 8;
                        let rs1 = ((inst >> 7) & 0x7) + 8;
                        // offset[5:3|2|6] = isnt[12:10|6|5]
                        let offset = ((inst << 1) & 0x40) // imm[6]
                            | ((inst >> 7) & 0x38) // imm[5:3]
                            | ((inst >> 4) & 0x4); // imm[2]
                        let addr = self.xregs.read(rs1).wrapping_add(offset);
                        self.write(addr, self.xregs.read(rs2), WORD)?;
                    }
                    0x7 => {
                        // c.sd
                        // Expands to sd rs2, offset(rs1), where rs2=rs2'+8 and rs1=rs1'+8.
                        inst_count!(self, "c.sd");

                        let rs2 = ((inst >> 2) & 0x7) + 8;
                        let rs1 = ((inst >> 7) & 0x7) + 8;
                        // offset[5:3|7:6] = isnt[12:10|6:5]
                        let offset = ((inst << 1) & 0xc0) // imm[7:6]
                            | ((inst >> 7) & 0x38); // imm[5:3]
                        let addr = self.xregs.read(rs1).wrapping_add(offset);
                        self.write(addr, self.xregs.read(rs2), DOUBLEWORD)?;
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction);
                    }
                }
            }
            1 => {
                // Quadrant 1.
                match funct3 {
                    0x0 => {
                        // c.addi
                        // Expands to addi rd, rd, nzimm.
                        inst_count!(self, "c.addi");

                        let rd = (inst >> 7) & 0x1f;
                        // nzimm[5|4:0] = inst[12|6:2]
                        let mut nzimm = ((inst >> 7) & 0x20) | ((inst >> 2) & 0x1f);
                        // Sign-extended.
                        nzimm = match (nzimm & 0x20) == 0 {
                            true => nzimm,
                            false => (0xc0 | nzimm) as i8 as i64 as u64,
                        };
                        if rd != 0 {
                            self.xregs
                                .write(rd, self.xregs.read(rd).wrapping_add(nzimm));
                        }
                    }
                    0x1 => {
                        // c.addiw
                        // Expands to addiw rd, rd, imm
                        // "The immediate can be zero for C.ADDIW, where this corresponds to sext.w
                        // rd"
                        inst_count!(self, "c.addiw");

                        let rd = (inst >> 7) & 0x1f;
                        // imm[5|4:0] = inst[12|6:2]
                        let mut imm = ((inst >> 7) & 0x20) | ((inst >> 2) & 0x1f);
                        // Sign-extended.
                        imm = match (imm & 0x20) == 0 {
                            true => imm,
                            false => (0xc0 | imm) as i8 as i64 as u64,
                        };
                        if rd != 0 {
                            self.xregs.write(
                                rd,
                                self.xregs.read(rd).wrapping_add(imm) as i32 as i64 as u64,
                            );
                        }
                    }
                    0x2 => {
                        // c.li
                        // Expands to addi rd, x0, imm.
                        inst_count!(self, "c.li");

                        let rd = (inst >> 7) & 0x1f;
                        // imm[5|4:0] = inst[12|6:2]
                        let mut imm = ((inst >> 7) & 0x20) | ((inst >> 2) & 0x1f);
                        // Sign-extended.
                        imm = match (imm & 0x20) == 0 {
                            true => imm,
                            false => (0xc0 | imm) as i8 as i64 as u64,
                        };
                        if rd != 0 {
                            self.xregs.write(rd, imm);
                        }
                    }
                    0x3 => {
                        let rd = (inst >> 7) & 0x1f;
                        match rd {
                            0 => {}
                            2 => {
                                // c.addi16sp
                                // Expands to addi x2, x2, nzimm
                                inst_count!(self, "c.addi16sp");

                                // nzimm[9|4|6|8:7|5] = inst[12|6|5|4:3|2]
                                let mut nzimm = ((inst >> 3) & 0x200) // nzimm[9]
                                    | ((inst >> 2) & 0x10) // nzimm[4]
                                    | ((inst << 1) & 0x40) // nzimm[6]
                                    | ((inst << 4) & 0x180) // nzimm[8:7]
                                    | ((inst << 3) & 0x20); // nzimm[5]
                                nzimm = match (nzimm & 0x200) == 0 {
                                    true => nzimm,
                                    // Sign-extended.
                                    false => (0xfc00 | nzimm) as i16 as i32 as i64 as u64,
                                };
                                if nzimm != 0 {
                                    self.xregs.write(2, self.xregs.read(2).wrapping_add(nzimm));
                                }
                            }
                            _ => {
                                // c.lui
                                // Expands to lui rd, nzimm.
                                inst_count!(self, "c.lui");

                                // nzimm[17|16:12] = inst[12|6:2]
                                let mut nzimm = ((inst << 5) & 0x20000) | ((inst << 10) & 0x1f000);
                                // Sign-extended.
                                nzimm = match (nzimm & 0x20000) == 0 {
                                    true => nzimm,
                                    false => (0xfffc0000 | nzimm) as i32 as i64 as u64,
                                };
                                if nzimm != 0 {
                                    self.xregs.write(rd, nzimm);
                                }
                            }
                        }
                    }
                    0x4 => {
                        let funct2 = (inst >> 10) & 0x3;
                        match funct2 {
                            0x0 => {
                                // c.srli
                                // Expands to srli rd, rd, shamt, where rd=rd'+8.
                                inst_count!(self, "c.srli");

                                let rd = ((inst >> 7) & 0b111) + 8;
                                // shamt[5|4:0] = inst[12|6:2]
                                let shamt = ((inst >> 7) & 0x20) | ((inst >> 2) & 0x1f);
                                self.xregs.write(rd, self.xregs.read(rd) >> shamt);
                            }
                            0x1 => {
                                // c.srai
                                // Expands to srai rd, rd, shamt, where rd=rd'+8.
                                inst_count!(self, "c.srai");

                                let rd = ((inst >> 7) & 0b111) + 8;
                                // shamt[5|4:0] = inst[12|6:2]
                                let shamt = ((inst >> 7) & 0x20) | ((inst >> 2) & 0x1f);
                                self.xregs
                                    .write(rd, ((self.xregs.read(rd) as i64) >> shamt) as u64);
                            }
                            0x2 => {
                                // c.andi
                                // Expands to andi rd, rd, imm, where rd=rd'+8.
                                inst_count!(self, "c.andi");

                                let rd = ((inst >> 7) & 0b111) + 8;
                                // imm[5|4:0] = inst[12|6:2]
                                let mut imm = ((inst >> 7) & 0x20) | ((inst >> 2) & 0x1f);
                                // Sign-extended.
                                imm = match (imm & 0x20) == 0 {
                                    true => imm,
                                    false => (0xc0 | imm) as i8 as i64 as u64,
                                };
                                self.xregs.write(rd, self.xregs.read(rd) & imm);
                            }
                            0x3 => {
                                match ((inst >> 12) & 0b1, (inst >> 5) & 0b11) {
                                    (0x0, 0x0) => {
                                        // c.sub
                                        // Expands to sub rd, rd, rs2, rd=rd'+8 and rs2=rs2'+8.
                                        inst_count!(self, "c.sub");

                                        let rd = ((inst >> 7) & 0b111) + 8;
                                        let rs2 = ((inst >> 2) & 0b111) + 8;
                                        self.xregs.write(
                                            rd,
                                            self.xregs.read(rd).wrapping_sub(self.xregs.read(rs2)),
                                        );
                                    }
                                    (0x0, 0x1) => {
                                        // c.xor
                                        // Expands to xor rd, rd, rs2, rd=rd'+8 and rs2=rs2'+8.
                                        inst_count!(self, "c.xor");

                                        let rd = ((inst >> 7) & 0b111) + 8;
                                        let rs2 = ((inst >> 2) & 0b111) + 8;
                                        self.xregs
                                            .write(rd, self.xregs.read(rd) ^ self.xregs.read(rs2));
                                    }
                                    (0x0, 0x2) => {
                                        // c.or
                                        // Expands to or rd, rd, rs2, rd=rd'+8 and rs2=rs2'+8.
                                        inst_count!(self, "c.or");

                                        let rd = ((inst >> 7) & 0b111) + 8;
                                        let rs2 = ((inst >> 2) & 0b111) + 8;
                                        self.xregs
                                            .write(rd, self.xregs.read(rd) | self.xregs.read(rs2));
                                    }
                                    (0x0, 0x3) => {
                                        // c.and
                                        // Expands to and rd, rd, rs2, rd=rd'+8 and rs2=rs2'+8.
                                        inst_count!(self, "c.and");

                                        let rd = ((inst >> 7) & 0b111) + 8;
                                        let rs2 = ((inst >> 2) & 0b111) + 8;
                                        self.xregs
                                            .write(rd, self.xregs.read(rd) & self.xregs.read(rs2));
                                    }
                                    (0x1, 0x0) => {
                                        // c.subw
                                        // Expands to subw rd, rd, rs2, rd=rd'+8 and rs2=rs2'+8.
                                        inst_count!(self, "c.subw");

                                        let rd = ((inst >> 7) & 0b111) + 8;
                                        let rs2 = ((inst >> 2) & 0b111) + 8;
                                        self.xregs.write(
                                            rd,
                                            self.xregs.read(rd).wrapping_sub(self.xregs.read(rs2))
                                                as i32
                                                as i64
                                                as u64,
                                        );
                                    }
                                    (0x1, 0x1) => {
                                        // c.addw
                                        // Expands to addw rd, rd, rs2, rd=rd'+8 and rs2=rs2'+8.
                                        inst_count!(self, "c.addw");

                                        let rd = ((inst >> 7) & 0b111) + 8;
                                        let rs2 = ((inst >> 2) & 0b111) + 8;
                                        self.xregs.write(
                                            rd,
                                            self.xregs.read(rd).wrapping_add(self.xregs.read(rs2))
                                                as i32
                                                as i64
                                                as u64,
                                        );
                                    }
                                    _ => {
                                        return Err(Exception::IllegalInstruction);
                                    }
                                }
                            }
                            _ => {
                                return Err(Exception::IllegalInstruction);
                            }
                        }
                    }
                    0x5 => {
                        // c.j
                        // Expands to jal x0, offset.
                        inst_count!(self, "c.j");

                        // offset[11|4|9:8|10|6|7|3:1|5] = inst[12|11|10:9|8|7|6|5:3|2]
                        let mut offset = ((inst >> 1) & 0x800) // offset[11]
                            | ((inst << 2) & 0x400) // offset[10]
                            | ((inst >> 1) & 0x300) // offset[9:8]
                            | ((inst << 1) & 0x80) // offset[7]
                            | ((inst >> 1) & 0x40) // offset[6]
                            | ((inst << 3) & 0x20) // offset[5]
                            | ((inst >> 7) & 0x10) // offset[4]
                            | ((inst >> 2) & 0xe); // offset[3:1]
                                                   // Sign-extended.
                        offset = match (offset & 0x800) == 0 {
                            true => offset,
                            false => (0xf000 | offset) as i16 as i64 as u64,
                        };
                        self.pc = self.pc.wrapping_add(offset).wrapping_sub(2);
                    }
                    0x6 => {
                        // c.beqz
                        // Expands to beq rs1, x0, offset, rs1=rs1'+8.
                        inst_count!(self, "c.beqz");

                        let rs1 = ((inst >> 7) & 0b111) + 8;
                        // offset[8|4:3|7:6|2:1|5] = inst[12|11:10|6:5|4:3|2]
                        let mut offset = ((inst >> 4) & 0x100) // offset[8]
                            | ((inst << 1) & 0xc0) // offset[7:6]
                            | ((inst << 3) & 0x20) // offset[5]
                            | ((inst >> 7) & 0x18) // offset[4:3]
                            | ((inst >> 2) & 0x6); // offset[2:1]
                                                   // Sign-extended.
                        offset = match (offset & 0x100) == 0 {
                            true => offset,
                            false => (0xfe00 | offset) as i16 as i64 as u64,
                        };
                        if self.xregs.read(rs1) == 0 {
                            self.pc = self.pc.wrapping_add(offset).wrapping_sub(2);
                        }
                    }
                    0x7 => {
                        // c.bnez
                        // Expands to bne rs1, x0, offset, rs1=rs1'+8.
                        inst_count!(self, "c.bnez");

                        let rs1 = ((inst >> 7) & 0b111) + 8;
                        // offset[8|4:3|7:6|2:1|5] = inst[12|11:10|6:5|4:3|2]
                        let mut offset = ((inst >> 4) & 0x100) // offset[8]
                            | ((inst << 1) & 0xc0) // offset[7:6]
                            | ((inst << 3) & 0x20) // offset[5]
                            | ((inst >> 7) & 0x18) // offset[4:3]
                            | ((inst >> 2) & 0x6); // offset[2:1]
                                                   // Sign-extended.
                        offset = match (offset & 0x100) == 0 {
                            true => offset,
                            false => (0xfe00 | offset) as i16 as i64 as u64,
                        };
                        if self.xregs.read(rs1) != 0 {
                            self.pc = self.pc.wrapping_add(offset).wrapping_sub(2);
                        }
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction);
                    }
                }
            }
            2 => {
                // Quadrant 2.
                match funct3 {
                    0x0 => {
                        // c.slli
                        // Expands to slli rd, rd, shamt.
                        inst_count!(self, "c.slli");

                        let rd = (inst >> 7) & 0x1f;
                        // shamt[5|4:0] = inst[12|6:2]
                        let shamt = ((inst >> 7) & 0x20) | ((inst >> 2) & 0x1f);
                        if rd != 0 {
                            self.xregs.write(rd, self.xregs.read(rd) << shamt);
                        }
                    }
                    0x1 => {
                        // c.fldsp
                        // Expands to fld rd, offset(x2).
                        inst_count!(self, "c.fldsp");

                        let rd = (inst >> 7) & 0x1f;
                        // offset[5|4:3|8:6] = inst[12|6:5|4:2]
                        let offset = ((inst << 4) & 0x1c0) // offset[8:6]
                            | ((inst >> 7) & 0x20) // offset[5]
                            | ((inst >> 2) & 0x18); // offset[4:3]
                        let val =
                            f64::from_bits(self.read(self.xregs.read(2) + offset, DOUBLEWORD)?);
                        self.fregs.write(rd, val);
                    }
                    0x2 => {
                        // c.lwsp
                        // Expands to lw rd, offset(x2).
                        inst_count!(self, "c.lwsp");

                        let rd = (inst >> 7) & 0x1f;
                        // offset[5|4:2|7:6] = inst[12|6:4|3:2]
                        let offset = ((inst << 4) & 0xc0) // offset[7:6]
                            | ((inst >> 7) & 0x20) // offset[5]
                            | ((inst >> 2) & 0x1c); // offset[4:2]
                        let val = self.read(self.xregs.read(2).wrapping_add(offset), WORD)?;
                        self.xregs.write(rd, val as i32 as i64 as u64);
                    }
                    0x3 => {
                        // c.ldsp
                        // Expands to ld rd, offset(x2).
                        inst_count!(self, "c.ldsp");

                        let rd = (inst >> 7) & 0x1f;
                        // offset[5|4:3|8:6] = inst[12|6:5|4:2]
                        let offset = ((inst << 4) & 0x1c0) // offset[8:6]
                            | ((inst >> 7) & 0x20) // offset[5]
                            | ((inst >> 2) & 0x18); // offset[4:3]
                        let val = self.read(self.xregs.read(2).wrapping_add(offset), DOUBLEWORD)?;
                        self.xregs.write(rd, val);
                    }
                    0x4 => {
                        match ((inst >> 12) & 0x1, (inst >> 2) & 0x1f) {
                            (0, 0) => {
                                // c.jr
                                // Expands to jalr x0, 0(rs1).
                                inst_count!(self, "c.jr");

                                let rs1 = (inst >> 7) & 0x1f;
                                if rs1 != 0 {
                                    self.pc = self.xregs.read(rs1);
                                }
                            }
                            (0, _) => {
                                // c.mv
                                // Expands to add rd, x0, rs2.
                                inst_count!(self, "c.mv");

                                let rd = (inst >> 7) & 0x1f;
                                let rs2 = (inst >> 2) & 0x1f;
                                if rs2 != 0 {
                                    self.xregs.write(rd, self.xregs.read(rs2));
                                }
                            }
                            (1, 0) => {
                                let rd = (inst >> 7) & 0x1f;
                                if rd == 0 {
                                    // c.ebreak
                                    // Expands to ebreak.
                                    inst_count!(self, "c.ebreak");

                                    return Err(Exception::Breakpoint);
                                } else {
                                    // c.jalr
                                    // Expands to jalr x1, 0(rs1).
                                    inst_count!(self, "c.jalr");

                                    let rs1 = (inst >> 7) & 0x1f;
                                    // Don't add 2 because the pc already moved on.
                                    let t = self.pc;
                                    self.pc = self.xregs.read(rs1);
                                    self.xregs.write(1, t);
                                }
                            }
                            (1, _) => {
                                // c.add
                                // Expands to add rd, rd, rs2.
                                inst_count!(self, "c.add");

                                let rd = (inst >> 7) & 0x1f;
                                let rs2 = (inst >> 2) & 0x1f;
                                if rs2 != 0 {
                                    self.xregs.write(
                                        rd,
                                        self.xregs.read(rd).wrapping_add(self.xregs.read(rs2)),
                                    );
                                }
                            }
                            (_, _) => {
                                return Err(Exception::IllegalInstruction);
                            }
                        }
                    }
                    0x5 => {
                        // c.fsdsp
                        // Expands to fsd rs2, offset(x2).
                        inst_count!(self, "c.fsdsp");

                        let rs2 = (inst >> 2) & 0x1f;
                        // offset[5:3|8:6] = isnt[12:10|9:7]
                        let offset = ((inst >> 1) & 0x1c0) // offset[8:6]
                            | ((inst >> 7) & 0x38); // offset[5:3]
                        let addr = self.xregs.read(2).wrapping_add(offset);
                        self.write(addr, self.fregs.read(rs2).to_bits(), DOUBLEWORD)?;
                    }
                    0x6 => {
                        // c.swsp
                        // Expands to sw rs2, offset(x2).
                        inst_count!(self, "c.swsp");

                        let rs2 = (inst >> 2) & 0x1f;
                        // offset[5:2|7:6] = inst[12:9|8:7]
                        let offset = ((inst >> 1) & 0xc0) // offset[7:6]
                            | ((inst >> 7) & 0x3c); // offset[5:2]
                        let addr = self.xregs.read(2).wrapping_add(offset);
                        self.write(addr, self.xregs.read(rs2), WORD)?;
                    }
                    0x7 => {
                        // c.sdsp
                        // Expands to sd rs2, offset(x2).
                        inst_count!(self, "c.sdsp");

                        let rs2 = (inst >> 2) & 0x1f;
                        // offset[5:3|8:6] = isnt[12:10|9:7]
                        let offset = ((inst >> 1) & 0x1c0) // offset[8:6]
                            | ((inst >> 7) & 0x38); // offset[5:3]
                        let addr = self.xregs.read(2).wrapping_add(offset);
                        self.write(addr, self.xregs.read(rs2), DOUBLEWORD)?;
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction);
                    }
                }
            }
            _ => {
                return Err(Exception::IllegalInstruction);
            }
        }
        Ok(inst)
    }

    /// Execute a general-purpose instruction. Raises an exception if something is wrong,
    /// otherwise, returns a fetched instruction. It also increments the program counter by 4 bytes.
    fn execute_general(&mut self) -> Result<u64, Exception> {
        // 1. Fetch.
        let inst = self.fetch(WORD)?;

        // Add 4 bytes to the program counter.
        self.pc += 4;

        // 2. Decode.
        let opcode = inst & 0x0000007f;
        let rd = (inst & 0x00000f80) >> 7;
        let rs1 = (inst & 0x000f8000) >> 15;
        let rs2 = (inst & 0x01f00000) >> 20;
        let funct3 = (inst & 0x00007000) >> 12;
        let funct7 = (inst & 0xfe000000) >> 25;

        // 3. Execute.
        match opcode {
            0x03 => {
                // RV32I and RV64I
                // imm[11:0] = inst[31:20]
                let offset = ((inst as i32 as i64) >> 20) as u64;
                let addr = self.xregs.read(rs1).wrapping_add(offset);
                match funct3 {
                    0x0 => {
                        // lb
                        inst_count!(self, "lb");

                        let val = self.read(addr, BYTE)?;
                        self.xregs.write(rd, val as i8 as i64 as u64);
                    }
                    0x1 => {
                        // lh
                        inst_count!(self, "lh");

                        let val = self.read(addr, HALFWORD)?;
                        self.xregs.write(rd, val as i16 as i64 as u64);
                    }
                    0x2 => {
                        // lw
                        inst_count!(self, "lw");

                        let val = self.read(addr, WORD)?;
                        self.xregs.write(rd, val as i32 as i64 as u64);
                    }
                    0x3 => {
                        // ld
                        inst_count!(self, "ld");

                        let val = self.read(addr, DOUBLEWORD)?;
                        self.xregs.write(rd, val);
                    }
                    0x4 => {
                        // lbu
                        inst_count!(self, "lbu");

                        let val = self.read(addr, BYTE)?;
                        self.xregs.write(rd, val);
                    }
                    0x5 => {
                        // lhu
                        inst_count!(self, "lhu");

                        let val = self.read(addr, HALFWORD)?;
                        self.xregs.write(rd, val);
                    }
                    0x6 => {
                        // lwu
                        inst_count!(self, "lwu");

                        let val = self.read(addr, WORD)?;
                        self.xregs.write(rd, val);
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction);
                    }
                }
            }
            0x07 => {
                // RV32D and RV64D
                // imm[11:0] = inst[31:20]
                let offset = ((inst as i32 as i64) >> 20) as u64;
                let addr = self.xregs.read(rs1).wrapping_add(offset);
                match funct3 {
                    0x2 => {
                        // flw
                        inst_count!(self, "flw");

                        let val = f32::from_bits(self.read(addr, WORD)? as u32);
                        self.fregs.write(rd, val as f64);
                    }
                    0x3 => {
                        // fld
                        inst_count!(self, "fld");

                        let val = f64::from_bits(self.read(addr, DOUBLEWORD)?);
                        self.fregs.write(rd, val);
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction);
                    }
                }
            }
            0x0f => {
                // RV32I and RV64I
                // fence instructions are not supportted yet because this emulator executes an
                // instruction sequentially on a single thread.
                // fence.i is a part of the Zifencei extension.
                match funct3 {
                    0x0 => {
                        // fence
                        inst_count!(self, "fence");
                    }
                    0x1 => {
                        // fence.i
                        inst_count!(self, "fence.i");
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction);
                    }
                }
            }
            0x13 => {
                // RV32I and RV64I
                // imm[11:0] = inst[31:20]
                let imm = ((inst as i32 as i64) >> 20) as u64;
                let funct6 = funct7 >> 1;
                match funct3 {
                    0x0 => {
                        // addi
                        inst_count!(self, "addi");

                        self.xregs.write(rd, self.xregs.read(rs1).wrapping_add(imm));
                    }
                    0x1 => {
                        // slli
                        inst_count!(self, "slli");

                        // shamt size is 5 bits for RV32I and 6 bits for RV64I.
                        let shamt = (inst >> 20) & 0x3f;
                        self.xregs.write(rd, self.xregs.read(rs1) << shamt);
                    }
                    0x2 => {
                        // slti
                        inst_count!(self, "slti");

                        self.xregs.write(
                            rd,
                            if (self.xregs.read(rs1) as i64) < (imm as i64) {
                                1
                            } else {
                                0
                            },
                        );
                    }
                    0x3 => {
                        // sltiu
                        inst_count!(self, "sltiu");

                        self.xregs
                            .write(rd, if self.xregs.read(rs1) < imm { 1 } else { 0 });
                    }
                    0x4 => {
                        // xori
                        inst_count!(self, "xori");

                        self.xregs.write(rd, self.xregs.read(rs1) ^ imm);
                    }
                    0x5 => {
                        match funct6 {
                            0x00 => {
                                // srli
                                inst_count!(self, "srli");

                                // shamt size is 5 bits for RV32I and 6 bits for RV64I.
                                let shamt = (inst >> 20) & 0x3f;
                                self.xregs.write(rd, self.xregs.read(rs1) >> shamt);
                            }
                            0x10 => {
                                // srai
                                inst_count!(self, "srai");

                                // shamt size is 5 bits for RV32I and 6 bits for RV64I.
                                let shamt = (inst >> 20) & 0x3f;
                                self.xregs
                                    .write(rd, ((self.xregs.read(rs1) as i64) >> shamt) as u64);
                            }
                            _ => {
                                return Err(Exception::IllegalInstruction);
                            }
                        }
                    }
                    0x6 => {
                        // ori
                        inst_count!(self, "ori");

                        self.xregs.write(rd, self.xregs.read(rs1) | imm);
                    }
                    0x7 => {
                        // andi
                        inst_count!(self, "andi");

                        self.xregs.write(rd, self.xregs.read(rs1) & imm);
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction);
                    }
                }
            }
            0x17 => {
                // RV32I
                // auipc
                inst_count!(self, "auipc");

                // AUIPC forms a 32-bit offset from the 20-bit U-immediate, filling
                // in the lowest 12 bits with zeros.
                // imm[31:12] = inst[31:12]
                let imm = (inst & 0xfffff000) as i32 as i64 as u64;
                self.xregs
                    .write(rd, self.pc.wrapping_add(imm).wrapping_sub(4));
            }
            0x1b => {
                // RV64I
                // imm[11:0] = inst[31:20]
                let imm = ((inst as i32 as i64) >> 20) as u64;
                match funct3 {
                    0x0 => {
                        // addiw
                        inst_count!(self, "addiw");

                        self.xregs.write(
                            rd,
                            self.xregs.read(rs1).wrapping_add(imm) as i32 as i64 as u64,
                        );
                    }
                    0x1 => {
                        // slliw
                        inst_count!(self, "slliw");

                        // "SLLIW, SRLIW, and SRAIW encodings with imm[5] ̸= 0 are reserved."
                        let shamt = (imm & 0x1f) as u32;
                        self.xregs
                            .write(rd, (self.xregs.read(rs1) << shamt) as i32 as i64 as u64);
                    }
                    0x5 => {
                        match funct7 {
                            0x00 => {
                                // srliw
                                inst_count!(self, "srliw");

                                // "SLLIW, SRLIW, and SRAIW encodings with imm[5] ̸= 0 are reserved."
                                let shamt = (imm & 0x1f) as u32;
                                self.xregs.write(
                                    rd,
                                    ((self.xregs.read(rs1) as u32) >> shamt) as i32 as i64 as u64,
                                )
                            }
                            0x20 => {
                                // sraiw
                                inst_count!(self, "sraiw");

                                // "SLLIW, SRLIW, and SRAIW encodings with imm[5] ̸= 0 are reserved."
                                let shamt = (imm & 0x1f) as u32;
                                self.xregs.write(
                                    rd,
                                    ((self.xregs.read(rs1) as i32) >> shamt) as i64 as u64,
                                );
                            }
                            _ => {
                                return Err(Exception::IllegalInstruction);
                            }
                        }
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction);
                    }
                }
            }
            0x23 => {
                // RV32I
                // offset[11:5|4:0] = inst[31:25|11:7]
                let offset =
                    (((inst & 0xfe000000) as i32 as i64 >> 20) as u64) | ((inst >> 7) & 0x1f);
                let addr = self.xregs.read(rs1).wrapping_add(offset);
                match funct3 {
                    0x0 => {
                        // sb
                        inst_count!(self, "sb");

                        self.write(addr, self.xregs.read(rs2), BYTE)?
                    }
                    0x1 => {
                        // sh
                        inst_count!(self, "sh");

                        self.write(addr, self.xregs.read(rs2), HALFWORD)?
                    }
                    0x2 => {
                        // sw
                        inst_count!(self, "sw");

                        self.write(addr, self.xregs.read(rs2), WORD)?
                    }
                    0x3 => {
                        // sd
                        inst_count!(self, "sd");

                        self.write(addr, self.xregs.read(rs2), DOUBLEWORD)?
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction);
                    }
                }
            }
            0x27 => {
                // RV32F and RV64F
                // offset[11:5|4:0] = inst[31:25|11:7]
                let offset = ((((inst as i32 as i64) >> 20) as u64) & 0xfe0) | ((inst >> 7) & 0x1f);
                let addr = self.xregs.read(rs1).wrapping_add(offset);
                match funct3 {
                    0x2 => {
                        // fsw
                        inst_count!(self, "fsw");

                        self.write(addr, (self.fregs.read(rs2) as f32).to_bits() as u64, WORD)?
                    }
                    0x3 => {
                        // fsd
                        inst_count!(self, "fsd");

                        self.write(addr, self.fregs.read(rs2).to_bits() as u64, DOUBLEWORD)?
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction);
                    }
                }
            }
            0x2f => {
                // RV32A and RV64A
                let funct5 = (funct7 & 0b1111100) >> 2;
                // TODO: Handle `aq` and `rl`.
                let _aq = (funct7 & 0b0000010) >> 1; // acquire access
                let _rl = funct7 & 0b0000001; // release access
                match (funct3, funct5) {
                    (0x2, 0x00) => {
                        // amoadd.w
                        inst_count!(self, "amoadd.w");

                        let addr = self.xregs.read(rs1);
                        // "For AMOs, the A extension requires that the address held in rs1 be
                        // naturally aligned to the size of the operand (i.e., eight-byte aligned
                        // for 64-bit words and four-byte aligned for 32-bit words). If the
                        // address is not naturally aligned, an address-misaligned exception or
                        // an access-fault exception will be generated."
                        if addr % 4 != 0 {
                            return Err(Exception::LoadAddressMisaligned);
                        }
                        let t = self.read(addr, WORD)?;
                        self.write(addr, t.wrapping_add(self.xregs.read(rs2)), WORD)?;
                        self.xregs.write(rd, t as i32 as i64 as u64);
                    }
                    (0x3, 0x00) => {
                        // amoadd.d
                        inst_count!(self, "amoadd.d");

                        let addr = self.xregs.read(rs1);
                        if addr % 8 != 0 {
                            return Err(Exception::LoadAddressMisaligned);
                        }
                        let t = self.read(addr, DOUBLEWORD)?;
                        self.write(addr, t.wrapping_add(self.xregs.read(rs2)), DOUBLEWORD)?;
                        self.xregs.write(rd, t);
                    }
                    (0x2, 0x01) => {
                        // amoswap.w
                        inst_count!(self, "amoswap.w");

                        let addr = self.xregs.read(rs1);
                        if addr % 4 != 0 {
                            return Err(Exception::LoadAddressMisaligned);
                        }
                        let t = self.read(addr, WORD)?;
                        self.write(addr, self.xregs.read(rs2), WORD)?;
                        self.xregs.write(rd, t as i32 as i64 as u64);
                    }
                    (0x3, 0x01) => {
                        // amoswap.d
                        inst_count!(self, "amoswap.d");

                        let addr = self.xregs.read(rs1);
                        if addr % 8 != 0 {
                            return Err(Exception::LoadAddressMisaligned);
                        }
                        let t = self.read(addr, DOUBLEWORD)?;
                        self.write(addr, self.xregs.read(rs2), DOUBLEWORD)?;
                        self.xregs.write(rd, t);
                    }
                    (0x2, 0x02) => {
                        // lr.w
                        inst_count!(self, "lr.w");

                        let addr = self.xregs.read(rs1);
                        // "For LR and SC, the A extension requires that the address held in rs1 be
                        // naturally aligned to the size of the operand (i.e., eight-byte aligned for
                        // 64-bit words and four-byte aligned for 32-bit words)."
                        if addr % 4 != 0 {
                            return Err(Exception::LoadAddressMisaligned);
                        }
                        let value = self.read(addr, WORD)?;
                        self.xregs.write(rd, value as i32 as i64 as u64);
                        self.reservation_set.push(addr);
                    }
                    (0x3, 0x02) => {
                        // lr.d
                        inst_count!(self, "lr.d");

                        let addr = self.xregs.read(rs1);
                        // "For LR and SC, the A extension requires that the address held in rs1 be
                        // naturally aligned to the size of the operand (i.e., eight-byte aligned for
                        // 64-bit words and four-byte aligned for 32-bit words)."
                        if addr % 8 != 0 {
                            return Err(Exception::LoadAddressMisaligned);
                        }
                        let value = self.read(addr, DOUBLEWORD)?;
                        self.xregs.write(rd, value);
                        self.reservation_set.push(addr);
                    }
                    (0x2, 0x03) => {
                        // sc.w
                        inst_count!(self, "sc.w");

                        let addr = self.xregs.read(rs1);
                        // "For LR and SC, the A extension requires that the address held in rs1 be
                        // naturally aligned to the size of the operand (i.e., eight-byte aligned for
                        // 64-bit words and four-byte aligned for 32-bit words)."
                        if addr % 4 != 0 {
                            return Err(Exception::StoreAMOAddressMisaligned);
                        }
                        if self.reservation_set.contains(&addr) {
                            // "Regardless of success or failure, executing an SC.W instruction
                            // invalidates any reservation held by this hart. "
                            self.reservation_set.retain(|&x| x != addr);
                            self.write(addr, self.xregs.read(rs2), WORD)?;
                            self.xregs.write(rd, 0);
                        } else {
                            self.reservation_set.retain(|&x| x != addr);
                            self.xregs.write(rd, 1);
                        };
                    }
                    (0x3, 0x03) => {
                        // sc.d
                        inst_count!(self, "sc.d");

                        let addr = self.xregs.read(rs1);
                        // "For LR and SC, the A extension requires that the address held in rs1 be
                        // naturally aligned to the size of the operand (i.e., eight-byte aligned for
                        // 64-bit words and four-byte aligned for 32-bit words)."
                        if addr % 8 != 0 {
                            return Err(Exception::StoreAMOAddressMisaligned);
                        }
                        if self.reservation_set.contains(&addr) {
                            self.reservation_set.retain(|&x| x != addr);
                            self.write(addr, self.xregs.read(rs2), DOUBLEWORD)?;
                            self.xregs.write(rd, 0);
                        } else {
                            self.reservation_set.retain(|&x| x != addr);
                            self.xregs.write(rd, 1);
                        }
                    }
                    (0x2, 0x04) => {
                        // amoxor.w
                        inst_count!(self, "amoxor.w");

                        let addr = self.xregs.read(rs1);
                        if addr % 4 != 0 {
                            return Err(Exception::LoadAddressMisaligned);
                        }
                        let t = self.read(addr, WORD)?;
                        self.write(
                            addr,
                            (t as i32 ^ (self.xregs.read(rs2) as i32)) as i64 as u64,
                            WORD,
                        )?;
                        self.xregs.write(rd, t as i32 as i64 as u64);
                    }
                    (0x3, 0x04) => {
                        // amoxor.d
                        inst_count!(self, "amoxor.d");

                        let addr = self.xregs.read(rs1);
                        if addr % 8 != 0 {
                            return Err(Exception::LoadAddressMisaligned);
                        }
                        let t = self.read(addr, DOUBLEWORD)?;
                        self.write(addr, t ^ self.xregs.read(rs2), DOUBLEWORD)?;
                        self.xregs.write(rd, t);
                    }
                    (0x2, 0x08) => {
                        // amoor.w
                        inst_count!(self, "amoor.w");

                        let addr = self.xregs.read(rs1);
                        if addr % 4 != 0 {
                            return Err(Exception::LoadAddressMisaligned);
                        }
                        let t = self.read(addr, WORD)?;
                        self.write(
                            addr,
                            (t as i32 | (self.xregs.read(rs2) as i32)) as i64 as u64,
                            WORD,
                        )?;
                        self.xregs.write(rd, t as i32 as i64 as u64);
                    }
                    (0x3, 0x08) => {
                        // amoor.d
                        inst_count!(self, "amoor.d");

                        let addr = self.xregs.read(rs1);
                        if addr % 8 != 0 {
                            return Err(Exception::LoadAddressMisaligned);
                        }
                        let t = self.read(addr, DOUBLEWORD)?;
                        self.write(addr, t | self.xregs.read(rs2), DOUBLEWORD)?;
                        self.xregs.write(rd, t);
                    }
                    (0x2, 0x0c) => {
                        // amoand.w
                        inst_count!(self, "amoand.w");

                        let addr = self.xregs.read(rs1);
                        if addr % 4 != 0 {
                            return Err(Exception::LoadAddressMisaligned);
                        }
                        let t = self.read(addr, WORD)?;
                        self.write(
                            addr,
                            (t as i32 & (self.xregs.read(rs2) as i32)) as u32 as u64,
                            WORD,
                        )?;
                        self.xregs.write(rd, t as i32 as i64 as u64);
                    }
                    (0x3, 0x0c) => {
                        // amoand.d
                        inst_count!(self, "amoand.d");

                        let addr = self.xregs.read(rs1);
                        if addr % 8 != 0 {
                            return Err(Exception::LoadAddressMisaligned);
                        }
                        let t = self.read(addr, DOUBLEWORD)?;
                        self.write(addr, t & self.xregs.read(rs1), DOUBLEWORD)?;
                        self.xregs.write(rd, t);
                    }
                    (0x2, 0x10) => {
                        // amomin.w
                        inst_count!(self, "amomin.w");

                        let addr = self.xregs.read(rs1);
                        if addr % 4 != 0 {
                            return Err(Exception::LoadAddressMisaligned);
                        }
                        let t = self.read(addr, WORD)?;
                        self.write(
                            addr,
                            cmp::min(t as i32, self.xregs.read(rs2) as i32) as i64 as u64,
                            WORD,
                        )?;
                        self.xregs.write(rd, t as i32 as i64 as u64);
                    }
                    (0x3, 0x10) => {
                        // amomin.d
                        inst_count!(self, "amomin.d");

                        let addr = self.xregs.read(rs1);
                        if addr % 8 != 0 {
                            return Err(Exception::LoadAddressMisaligned);
                        }
                        let t = self.read(addr, DOUBLEWORD)?;
                        self.write(
                            addr,
                            cmp::min(t as i64, self.xregs.read(rs2) as i64) as u64,
                            DOUBLEWORD,
                        )?;
                        self.xregs.write(rd, t as u64);
                    }
                    (0x2, 0x14) => {
                        // amomax.w
                        inst_count!(self, "amomax.w");

                        let addr = self.xregs.read(rs1);
                        if addr % 4 != 0 {
                            return Err(Exception::LoadAddressMisaligned);
                        }
                        let t = self.read(addr, WORD)?;
                        self.write(
                            addr,
                            cmp::max(t as i32, self.xregs.read(rs2) as i32) as i64 as u64,
                            WORD,
                        )?;
                        self.xregs.write(rd, t as i32 as i64 as u64);
                    }
                    (0x3, 0x14) => {
                        // amomax.d
                        inst_count!(self, "amomax.d");

                        let addr = self.xregs.read(rs1);
                        if addr % 8 != 0 {
                            return Err(Exception::LoadAddressMisaligned);
                        }
                        let t = self.read(addr, DOUBLEWORD)?;
                        self.write(
                            addr,
                            cmp::max(t as i64, self.xregs.read(rs2) as i64) as u64,
                            DOUBLEWORD,
                        )?;
                        self.xregs.write(rd, t);
                    }
                    (0x2, 0x18) => {
                        // amominu.w
                        inst_count!(self, "amominu.w");

                        let addr = self.xregs.read(rs1);
                        if addr % 4 != 0 {
                            return Err(Exception::LoadAddressMisaligned);
                        }
                        let t = self.read(addr, WORD)?;
                        self.write(
                            addr,
                            cmp::min(t as u32, self.xregs.read(rs2) as u32) as u64,
                            WORD,
                        )?;
                        self.xregs.write(rd, t as i32 as i64 as u64);
                    }
                    (0x3, 0x18) => {
                        // amominu.d
                        inst_count!(self, "amominu.d");

                        let addr = self.xregs.read(rs1);
                        if addr % 8 != 0 {
                            return Err(Exception::LoadAddressMisaligned);
                        }
                        let t = self.read(addr, DOUBLEWORD)?;
                        self.write(addr, cmp::min(t, self.xregs.read(rs2)), DOUBLEWORD)?;
                        self.xregs.write(rd, t);
                    }
                    (0x2, 0x1c) => {
                        // amomaxu.w
                        inst_count!(self, "amomaxu.w");

                        let addr = self.xregs.read(rs1);
                        if addr % 4 != 0 {
                            return Err(Exception::LoadAddressMisaligned);
                        }
                        let t = self.read(addr, WORD)?;
                        self.write(
                            addr,
                            cmp::max(t as u32, self.xregs.read(rs2) as u32) as u64,
                            WORD,
                        )?;
                        self.xregs.write(rd, t as i32 as i64 as u64);
                    }
                    (0x3, 0x1c) => {
                        // amomaxu.d
                        inst_count!(self, "amomaxu.d");

                        let addr = self.xregs.read(rs1);
                        if addr % 8 != 0 {
                            return Err(Exception::LoadAddressMisaligned);
                        }
                        let t = self.read(addr, DOUBLEWORD)?;
                        self.write(addr, cmp::max(t, self.xregs.read(rs2)), DOUBLEWORD)?;
                        self.xregs.write(rd, t);
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction);
                    }
                }
            }
            0x33 => {
                // RV64I and RV64M
                match (funct3, funct7) {
                    (0x0, 0x00) => {
                        // add
                        inst_count!(self, "add");

                        self.xregs
                            .write(rd, self.xregs.read(rs1).wrapping_add(self.xregs.read(rs2)));
                    }
                    (0x0, 0x01) => {
                        // mul
                        inst_count!(self, "mul");

                        self.xregs.write(
                            rd,
                            (self.xregs.read(rs1) as i64).wrapping_mul(self.xregs.read(rs2) as i64)
                                as u64,
                        );
                    }
                    (0x0, 0x20) => {
                        // sub
                        inst_count!(self, "sub");

                        self.xregs
                            .write(rd, self.xregs.read(rs1).wrapping_sub(self.xregs.read(rs2)));
                    }
                    (0x1, 0x00) => {
                        // sll
                        inst_count!(self, "sll");

                        // "SLL, SRL, and SRA perform logical left, logical right, and arithmetic
                        // right shifts on the value in register rs1 by the shift amount held in
                        // register rs2. In RV64I, only the low 6 bits of rs2 are considered for the
                        // shift amount."
                        let shamt = self.xregs.read(rs2) & 0x3f;
                        self.xregs.write(rd, self.xregs.read(rs1) << shamt);
                    }
                    (0x1, 0x01) => {
                        // mulh
                        inst_count!(self, "mulh");

                        // signed × signed
                        self.xregs.write(
                            rd,
                            ((self.xregs.read(rs1) as i64 as i128)
                                .wrapping_mul(self.xregs.read(rs2) as i64 as i128)
                                >> 64) as u64,
                        );
                    }
                    (0x2, 0x00) => {
                        // slt
                        inst_count!(self, "slt");

                        self.xregs.write(
                            rd,
                            if (self.xregs.read(rs1) as i64) < (self.xregs.read(rs2) as i64) {
                                1
                            } else {
                                0
                            },
                        );
                    }
                    (0x2, 0x01) => {
                        // mulhsu
                        inst_count!(self, "mulhsu");

                        // signed × unsigned
                        self.xregs.write(
                            rd,
                            ((self.xregs.read(rs1) as i64 as i128 as u128)
                                .wrapping_mul(self.xregs.read(rs2) as u128)
                                >> 64) as u64,
                        );
                    }
                    (0x3, 0x00) => {
                        // sltu
                        inst_count!(self, "sltu");

                        self.xregs.write(
                            rd,
                            if self.xregs.read(rs1) < self.xregs.read(rs2) {
                                1
                            } else {
                                0
                            },
                        );
                    }
                    (0x3, 0x01) => {
                        // mulhu
                        inst_count!(self, "mulhu");

                        // unsigned × unsigned
                        self.xregs.write(
                            rd,
                            ((self.xregs.read(rs1) as u128)
                                .wrapping_mul(self.xregs.read(rs2) as u128)
                                >> 64) as u64,
                        );
                    }
                    (0x4, 0x00) => {
                        // xor
                        inst_count!(self, "xor");

                        self.xregs
                            .write(rd, self.xregs.read(rs1) ^ self.xregs.read(rs2));
                    }
                    (0x4, 0x01) => {
                        // div
                        inst_count!(self, "div");

                        let dividend = self.xregs.read(rs1) as i64;
                        let divisor = self.xregs.read(rs2) as i64;
                        self.xregs.write(
                            rd,
                            if divisor == 0 {
                                // Division by zero
                                // Set DZ (Divide by Zero) flag to 1.
                                self.state.write_bit(FCSR, 3, 1);
                                // "The quotient of division by zero has all bits set"
                                u64::MAX
                            } else if dividend == i64::MIN && divisor == -1 {
                                // Overflow
                                // "The quotient of a signed division with overflow is equal to the
                                // dividend"
                                dividend as u64
                            } else {
                                // "division of rs1 by rs2, rounding towards zero"
                                dividend.wrapping_div(divisor) as u64
                            },
                        );
                    }
                    (0x5, 0x00) => {
                        // srl
                        inst_count!(self, "srl");

                        // "SLL, SRL, and SRA perform logical left, logical right, and arithmetic
                        // right shifts on the value in register rs1 by the shift amount held in
                        // register rs2. In RV64I, only the low 6 bits of rs2 are considered for the
                        // shift amount."
                        let shamt = self.xregs.read(rs2) & 0x3f;
                        self.xregs.write(rd, self.xregs.read(rs1) >> shamt);
                    }
                    (0x5, 0x01) => {
                        // divu
                        inst_count!(self, "divu");

                        let dividend = self.xregs.read(rs1);
                        let divisor = self.xregs.read(rs2);
                        self.xregs.write(
                            rd,
                            if divisor == 0 {
                                // Division by zero
                                // Set DZ (Divide by Zero) flag to 1.
                                self.state.write_bit(FCSR, 3, 1);
                                // "The quotient of division by zero has all bits set"
                                u64::MAX
                            } else {
                                // "division of rs1 by rs2, rounding towards zero"
                                dividend.wrapping_div(divisor)
                            },
                        );
                    }
                    (0x5, 0x20) => {
                        // sra
                        inst_count!(self, "sra");

                        // "SLL, SRL, and SRA perform logical left, logical right, and arithmetic
                        // right shifts on the value in register rs1 by the shift amount held in
                        // register rs2. In RV64I, only the low 6 bits of rs2 are considered for the
                        // shift amount."
                        let shamt = self.xregs.read(rs2) & 0x3f;
                        self.xregs
                            .write(rd, ((self.xregs.read(rs1) as i64) >> shamt) as u64);
                    }
                    (0x6, 0x00) => {
                        // or
                        inst_count!(self, "or");

                        self.xregs
                            .write(rd, self.xregs.read(rs1) | self.xregs.read(rs2));
                    }
                    (0x6, 0x01) => {
                        // rem
                        inst_count!(self, "rem");

                        let dividend = self.xregs.read(rs1) as i64;
                        let divisor = self.xregs.read(rs2) as i64;
                        self.xregs.write(
                            rd,
                            if divisor == 0 {
                                // Division by zero
                                // "the remainder of division by zero equals the dividend"
                                dividend as u64
                            } else if dividend == i64::MIN && divisor == -1 {
                                // Overflow
                                // "the remainder is zero"
                                0
                            } else {
                                // "provide the remainder of the corresponding division
                                // operation"
                                dividend.wrapping_rem(divisor) as u64
                            },
                        );
                    }
                    (0x7, 0x00) => {
                        // and
                        inst_count!(self, "and");

                        self.xregs
                            .write(rd, self.xregs.read(rs1) & self.xregs.read(rs2));
                    }
                    (0x7, 0x01) => {
                        // remu
                        inst_count!(self, "remu");

                        let dividend = self.xregs.read(rs1);
                        let divisor = self.xregs.read(rs2);
                        self.xregs.write(
                            rd,
                            if divisor == 0 {
                                // Division by zero
                                // "the remainder of division by zero equals the dividend"
                                dividend
                            } else {
                                // "provide the remainder of the corresponding division
                                // operation"
                                dividend.wrapping_rem(divisor)
                            },
                        );
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction);
                    }
                };
            }
            0x37 => {
                // RV32I
                // lui
                inst_count!(self, "lui");

                // "LUI places the U-immediate value in the top 20 bits of the destination
                // register rd, filling in the lowest 12 bits with zeros."
                self.xregs
                    .write(rd, (inst & 0xfffff000) as i32 as i64 as u64);
            }
            0x3b => {
                // RV64I and RV64M
                match (funct3, funct7) {
                    (0x0, 0x00) => {
                        // addw
                        inst_count!(self, "addw");

                        self.xregs.write(
                            rd,
                            self.xregs.read(rs1).wrapping_add(self.xregs.read(rs2)) as i32 as i64
                                as u64,
                        );
                    }
                    (0x0, 0x01) => {
                        // mulw
                        inst_count!(self, "mulw");

                        let n1 = self.xregs.read(rs1) as i32;
                        let n2 = self.xregs.read(rs2) as i32;
                        let result = n1.wrapping_mul(n2);
                        self.xregs.write(rd, result as i64 as u64);
                    }
                    (0x0, 0x20) => {
                        // subw
                        inst_count!(self, "subw");

                        self.xregs.write(
                            rd,
                            ((self.xregs.read(rs1).wrapping_sub(self.xregs.read(rs2))) as i32)
                                as u64,
                        );
                    }
                    (0x1, 0x00) => {
                        // sllw
                        inst_count!(self, "sllw");

                        // The shift amount is given by rs2[4:0].
                        let shamt = self.xregs.read(rs2) & 0x1f;
                        self.xregs
                            .write(rd, ((self.xregs.read(rs1)) << shamt) as i32 as i64 as u64);
                    }
                    (0x4, 0x01) => {
                        // divw
                        inst_count!(self, "divw");

                        let dividend = self.xregs.read(rs1) as i32;
                        let divisor = self.xregs.read(rs2) as i32;
                        self.xregs.write(
                            rd,
                            if divisor == 0 {
                                // Division by zero
                                // Set DZ (Divide by Zero) flag to 1.
                                self.state.write_bit(FCSR, 3, 1);
                                // "The quotient of division by zero has all bits set"
                                u64::MAX
                            } else if dividend == i32::MIN && divisor == -1 {
                                // Overflow
                                // "The quotient of a signed division with overflow is equal to the
                                // dividend"
                                dividend as i64 as u64
                            } else {
                                // "division of rs1 by rs2, rounding towards zero"
                                dividend.wrapping_div(divisor) as i64 as u64
                            },
                        );
                    }
                    (0x5, 0x00) => {
                        // srlw
                        inst_count!(self, "srlw");

                        // The shift amount is given by rs2[4:0].
                        let shamt = self.xregs.read(rs2) & 0x1f;
                        self.xregs.write(
                            rd,
                            ((self.xregs.read(rs1) as u32) >> shamt) as i32 as i64 as u64,
                        );
                    }
                    (0x5, 0x01) => {
                        // divuw
                        inst_count!(self, "divuw");

                        let dividend = self.xregs.read(rs1) as u32;
                        let divisor = self.xregs.read(rs2) as u32;
                        self.xregs.write(
                            rd,
                            if divisor == 0 {
                                // Division by zero
                                // Set DZ (Divide by Zero) flag to 1.
                                self.state.write_bit(FCSR, 3, 1);
                                // "The quotient of division by zero has all bits set"
                                u64::MAX
                            } else {
                                // "division of rs1 by rs2, rounding towards zero"
                                dividend.wrapping_div(divisor) as i32 as i64 as u64
                            },
                        );
                    }
                    (0x5, 0x20) => {
                        // sraw
                        inst_count!(self, "sraw");

                        // The shift amount is given by rs2[4:0].
                        let shamt = self.xregs.read(rs2) & 0x1f;
                        self.xregs
                            .write(rd, ((self.xregs.read(rs1) as i32) >> shamt) as i64 as u64);
                    }
                    (0x6, 0x01) => {
                        // remw
                        inst_count!(self, "remw");

                        let dividend = self.xregs.read(rs1) as i32;
                        let divisor = self.xregs.read(rs2) as i32;
                        self.xregs.write(
                            rd,
                            if divisor == 0 {
                                // Division by zero
                                // "the remainder of division by zero equals the dividend"
                                dividend as i64 as u64
                            } else if dividend == i32::MIN && divisor == -1 {
                                // Overflow
                                // "the remainder is zero"
                                0
                            } else {
                                // "provide the remainder of the corresponding division
                                // operation"
                                dividend.wrapping_rem(divisor) as i64 as u64
                            },
                        );
                    }
                    (0x7, 0x01) => {
                        // remuw
                        inst_count!(self, "remuw");

                        let dividend = self.xregs.read(rs1) as u32;
                        let divisor = self.xregs.read(rs2) as u32;
                        self.xregs.write(
                            rd,
                            if divisor == 0 {
                                // Division by zero
                                // "the remainder of division by zero equals the dividend"
                                dividend as u64
                            } else {
                                // "provide the remainder of the corresponding division
                                // operation"
                                dividend.wrapping_rem(divisor) as i32 as i64 as u64
                            },
                        );
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction);
                    }
                }
            }
            0x43 => {
                // RV32F and RV64F
                // TODO: support the rounding mode encoding (rm).
                let rs3 = ((inst & 0xf8000000) >> 27) as u64;
                let funct2 = (inst & 0x03000000) >> 25;
                match funct2 {
                    0x0 => {
                        // fmadd.s
                        inst_count!(self, "fmadd.s");

                        self.fregs.write(
                            rd,
                            (self.fregs.read(rs1) as f32)
                                .mul_add(self.fregs.read(rs2) as f32, self.fregs.read(rs3) as f32)
                                as f64,
                        );
                    }
                    0x1 => {
                        // fmadd.d
                        inst_count!(self, "fmadd.d");

                        self.fregs.write(
                            rd,
                            self.fregs
                                .read(rs1)
                                .mul_add(self.fregs.read(rs2), self.fregs.read(rs3)),
                        );
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction);
                    }
                }
            }
            0x47 => {
                // RV32F and RV64F
                // TODO: support the rounding mode encoding (rm).
                let rs3 = ((inst & 0xf8000000) >> 27) as u64;
                let funct2 = (inst & 0x03000000) >> 25;
                match funct2 {
                    0x0 => {
                        // fmsub.s
                        inst_count!(self, "fmsub.s");

                        self.fregs.write(
                            rd,
                            (self.fregs.read(rs1) as f32)
                                .mul_add(self.fregs.read(rs2) as f32, -self.fregs.read(rs3) as f32)
                                as f64,
                        );
                    }
                    0x1 => {
                        // fmsub.d
                        inst_count!(self, "fmsub.d");

                        self.fregs.write(
                            rd,
                            self.fregs
                                .read(rs1)
                                .mul_add(self.fregs.read(rs2), -self.fregs.read(rs3)),
                        );
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction);
                    }
                }
            }
            0x4b => {
                // RV32F and RV64F
                // TODO: support the rounding mode encoding (rm).
                let rs3 = ((inst & 0xf8000000) >> 27) as u64;
                let funct2 = (inst & 0x03000000) >> 25;
                match funct2 {
                    0x0 => {
                        // fnmadd.s
                        inst_count!(self, "fnmadd.s");

                        self.fregs.write(
                            rd,
                            (-self.fregs.read(rs1) as f32)
                                .mul_add(self.fregs.read(rs2) as f32, self.fregs.read(rs3) as f32)
                                as f64,
                        );
                    }
                    0x1 => {
                        // fnmadd.d
                        inst_count!(self, "fnmadd.d");

                        self.fregs.write(
                            rd,
                            (-self.fregs.read(rs1))
                                .mul_add(self.fregs.read(rs2), self.fregs.read(rs3)),
                        );
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction);
                    }
                }
            }
            0x4f => {
                // RV32F and RV64F
                // TODO: support the rounding mode encoding (rm).
                let rs3 = ((inst & 0xf8000000) >> 27) as u64;
                let funct2 = (inst & 0x03000000) >> 25;
                match funct2 {
                    0x0 => {
                        // fnmsub.s
                        inst_count!(self, "fnmsub.s");

                        self.fregs.write(
                            rd,
                            (-self.fregs.read(rs1) as f32)
                                .mul_add(self.fregs.read(rs2) as f32, -self.fregs.read(rs3) as f32)
                                as f64,
                        );
                    }
                    0x1 => {
                        // fnmsub.d
                        inst_count!(self, "fnmsub.d");

                        self.fregs.write(
                            rd,
                            (-self.fregs.read(rs1))
                                .mul_add(self.fregs.read(rs2), -self.fregs.read(rs3)),
                        );
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction);
                    }
                }
            }
            0x53 => {
                // RV32F and RV64F
                // TODO: support the rounding mode encoding (rm).
                // TODO: NaN Boxing of Narrower Values (Spec 12.2).
                // TODO: set exception flags.

                /*
                 * Floating-point instructions align with the IEEE 754 (1985).
                 * The format consist of three fields: a sign bit, a biased exponent, and a fraction.
                 *
                 * | sign(1) | exponent(8) | fraction(23) |
                 * Ok => {}
                 * 31                                     0
                 *
                 */

                // Check the frm field is valid.
                match self.state.read_bits(FCSR, 5..8) {
                    0b000 => {}
                    0b001 => {}
                    0b010 => {}
                    0b011 => {}
                    0b100 => {}
                    0b111 => {}
                    _ => {
                        return Err(Exception::IllegalInstruction);
                    }
                }

                match funct7 {
                    0x00 => {
                        // fadd.s
                        inst_count!(self, "fadd.s");

                        self.fregs.write(
                            rd,
                            (self.fregs.read(rs1) as f32 + self.fregs.read(rs2) as f32) as f64,
                        )
                    }
                    0x01 => {
                        // fadd.d
                        inst_count!(self, "fadd.d");

                        self.fregs
                            .write(rd, self.fregs.read(rs1) + self.fregs.read(rs2));
                    }
                    0x04 => {
                        // fsub.s
                        inst_count!(self, "fsub.s");

                        self.fregs.write(
                            rd,
                            (self.fregs.read(rs1) as f32 - self.fregs.read(rs2) as f32) as f64,
                        )
                    }
                    0x05 => {
                        // fsub.d
                        inst_count!(self, "fsub.d");

                        self.fregs
                            .write(rd, self.fregs.read(rs1) - self.fregs.read(rs2));
                    }
                    0x08 => {
                        // fmul.s
                        inst_count!(self, "fmul.s");

                        self.fregs.write(
                            rd,
                            (self.fregs.read(rs1) as f32 * self.fregs.read(rs2) as f32) as f64,
                        )
                    }
                    0x09 => {
                        // fmul.d
                        inst_count!(self, "fmul.d");

                        self.fregs
                            .write(rd, self.fregs.read(rs1) * self.fregs.read(rs2));
                    }
                    0x0c => {
                        // fdiv.s
                        inst_count!(self, "fdiv.s");

                        self.fregs.write(
                            rd,
                            (self.fregs.read(rs1) as f32 / self.fregs.read(rs2) as f32) as f64,
                        )
                    }
                    0x0d => {
                        // fdiv.d
                        inst_count!(self, "fdiv.d");

                        self.fregs
                            .write(rd, self.fregs.read(rs1) / self.fregs.read(rs2));
                    }
                    0x10 => {
                        match funct3 {
                            0x0 => {
                                // fsgnj.s
                                inst_count!(self, "fsgnj.s");

                                self.fregs
                                    .write(rd, self.fregs.read(rs1).copysign(self.fregs.read(rs2)));
                            }
                            0x1 => {
                                // fsgnjn.s
                                inst_count!(self, "fsgnjn.s");

                                self.fregs.write(
                                    rd,
                                    self.fregs.read(rs1).copysign(-self.fregs.read(rs2)),
                                );
                            }
                            0x2 => {
                                // fsgnjx.s
                                inst_count!(self, "fsgnjx.s");

                                let sign1 = (self.fregs.read(rs1) as f32).to_bits() & 0x80000000;
                                let sign2 = (self.fregs.read(rs2) as f32).to_bits() & 0x80000000;
                                let other = (self.fregs.read(rs1) as f32).to_bits() & 0x7fffffff;
                                self.fregs
                                    .write(rd, f32::from_bits((sign1 ^ sign2) | other) as f64);
                            }
                            _ => {
                                return Err(Exception::IllegalInstruction);
                            }
                        }
                    }
                    0x11 => {
                        match funct3 {
                            0x0 => {
                                // fsgnj.d
                                inst_count!(self, "fsgnj.d");

                                self.fregs
                                    .write(rd, self.fregs.read(rs1).copysign(self.fregs.read(rs2)));
                            }
                            0x1 => {
                                // fsgnjn.d
                                inst_count!(self, "fsgnjn.d");

                                self.fregs.write(
                                    rd,
                                    self.fregs.read(rs1).copysign(-self.fregs.read(rs2)),
                                );
                            }
                            0x2 => {
                                // fsgnjx.d
                                inst_count!(self, "fsgnjx.d");

                                let sign1 = self.fregs.read(rs1).to_bits() & 0x80000000_00000000;
                                let sign2 = self.fregs.read(rs2).to_bits() & 0x80000000_00000000;
                                let other = self.fregs.read(rs1).to_bits() & 0x7fffffff_ffffffff;
                                self.fregs
                                    .write(rd, f64::from_bits((sign1 ^ sign2) | other));
                            }
                            _ => {
                                return Err(Exception::IllegalInstruction);
                            }
                        }
                    }
                    0x14 => {
                        match funct3 {
                            0x0 => {
                                // fmin.s
                                inst_count!(self, "fmin.s");

                                self.fregs
                                    .write(rd, self.fregs.read(rs1).min(self.fregs.read(rs2)));
                            }
                            0x1 => {
                                // fmax.s
                                inst_count!(self, "fmax.s");

                                self.fregs
                                    .write(rd, self.fregs.read(rs1).max(self.fregs.read(rs2)));
                            }
                            _ => {
                                return Err(Exception::IllegalInstruction);
                            }
                        }
                    }
                    0x15 => {
                        match funct3 {
                            0x0 => {
                                // fmin.d
                                inst_count!(self, "fmin.d");

                                self.fregs
                                    .write(rd, self.fregs.read(rs1).min(self.fregs.read(rs2)));
                            }
                            0x1 => {
                                // fmax.d
                                inst_count!(self, "fmax.d");

                                self.fregs
                                    .write(rd, self.fregs.read(rs1).max(self.fregs.read(rs2)));
                            }
                            _ => {
                                return Err(Exception::IllegalInstruction);
                            }
                        }
                    }
                    0x20 => {
                        // fcvt.s.d
                        inst_count!(self, "fcvt.s.d");

                        self.fregs.write(rd, self.fregs.read(rs1));
                    }
                    0x21 => {
                        // fcvt.d.s
                        inst_count!(self, "fcvt.d.s");

                        self.fregs.write(rd, (self.fregs.read(rs1) as f32) as f64);
                    }
                    0x2c => {
                        // fsqrt.s
                        inst_count!(self, "fsqrt.s");

                        self.fregs
                            .write(rd, (self.fregs.read(rs1) as f32).sqrt() as f64);
                    }
                    0x2d => {
                        // fsqrt.d
                        inst_count!(self, "fsqrt.d");

                        self.fregs.write(rd, self.fregs.read(rs1).sqrt());
                    }
                    0x50 => {
                        match funct3 {
                            0x0 => {
                                // fle.s
                                inst_count!(self, "fle.s");

                                self.xregs.write(
                                    rd,
                                    if self.fregs.read(rs1) <= self.fregs.read(rs2) {
                                        1
                                    } else {
                                        0
                                    },
                                );
                            }
                            0x1 => {
                                // flt.s
                                inst_count!(self, "flt.s");

                                self.xregs.write(
                                    rd,
                                    if self.fregs.read(rs1) < self.fregs.read(rs2) {
                                        1
                                    } else {
                                        0
                                    },
                                );
                            }
                            0x2 => {
                                // feq.s
                                inst_count!(self, "feq.s");

                                self.xregs.write(
                                    rd,
                                    if self.fregs.read(rs1) == self.fregs.read(rs2) {
                                        1
                                    } else {
                                        0
                                    },
                                );
                            }
                            _ => {
                                return Err(Exception::IllegalInstruction);
                            }
                        }
                    }
                    0x51 => {
                        match funct3 {
                            0x0 => {
                                // fle.d
                                inst_count!(self, "fle.d");

                                self.xregs.write(
                                    rd,
                                    if self.fregs.read(rs1) <= self.fregs.read(rs2) {
                                        1
                                    } else {
                                        0
                                    },
                                );
                            }
                            0x1 => {
                                // flt.d
                                inst_count!(self, "flt.d");

                                self.xregs.write(
                                    rd,
                                    if self.fregs.read(rs1) < self.fregs.read(rs2) {
                                        1
                                    } else {
                                        0
                                    },
                                );
                            }
                            0x2 => {
                                // feq.d
                                inst_count!(self, "feq.d");

                                self.xregs.write(
                                    rd,
                                    if self.fregs.read(rs1) == self.fregs.read(rs2) {
                                        1
                                    } else {
                                        0
                                    },
                                );
                            }
                            _ => {
                                return Err(Exception::IllegalInstruction);
                            }
                        }
                    }
                    0x60 => {
                        match rs2 {
                            0x0 => {
                                // fcvt.w.s
                                inst_count!(self, "fcvt.w.s");

                                self.xregs.write(
                                    rd,
                                    ((self.fregs.read(rs1) as f32).round() as i32) as u64,
                                );
                            }
                            0x1 => {
                                // fcvt.wu.s
                                inst_count!(self, "fcvt.wu.s");

                                self.xregs.write(
                                    rd,
                                    (((self.fregs.read(rs1) as f32).round() as u32) as i32) as u64,
                                );
                            }
                            0x2 => {
                                // fcvt.l.s
                                inst_count!(self, "fcvt.l.s");

                                self.xregs
                                    .write(rd, (self.fregs.read(rs1) as f32).round() as u64);
                            }
                            0x3 => {
                                // fcvt.lu.s
                                inst_count!(self, "fcvt.lu.s");

                                self.xregs
                                    .write(rd, (self.fregs.read(rs1) as f32).round() as u64);
                            }
                            _ => {
                                return Err(Exception::IllegalInstruction);
                            }
                        }
                    }
                    0x61 => {
                        match rs2 {
                            0x0 => {
                                // fcvt.w.d
                                inst_count!(self, "fcvt.w.d");

                                self.xregs
                                    .write(rd, (self.fregs.read(rs1).round() as i32) as u64);
                            }
                            0x1 => {
                                // fcvt.wu.d
                                inst_count!(self, "fcvt.wu.d");

                                self.xregs.write(
                                    rd,
                                    ((self.fregs.read(rs1).round() as u32) as i32) as u64,
                                );
                            }
                            0x2 => {
                                // fcvt.l.d
                                inst_count!(self, "fcvt.l.d");

                                self.xregs.write(rd, self.fregs.read(rs1).round() as u64);
                            }
                            0x3 => {
                                // fcvt.lu.d
                                inst_count!(self, "fcvt.lu.d");

                                self.xregs.write(rd, self.fregs.read(rs1).round() as u64);
                            }
                            _ => {
                                return Err(Exception::IllegalInstruction);
                            }
                        }
                    }
                    0x68 => {
                        match rs2 {
                            0x0 => {
                                // fcvt.s.w
                                inst_count!(self, "fcvt.s.w");

                                self.fregs
                                    .write(rd, ((self.xregs.read(rs1) as i32) as f32) as f64);
                            }
                            0x1 => {
                                // fcvt.s.wu
                                inst_count!(self, "fcvt.s.wu");

                                self.fregs
                                    .write(rd, ((self.xregs.read(rs1) as u32) as f32) as f64);
                            }
                            0x2 => {
                                // fcvt.s.l
                                inst_count!(self, "fcvt.s.l");

                                self.fregs.write(rd, (self.xregs.read(rs1) as f32) as f64);
                            }
                            0x3 => {
                                // fcvt.s.lu
                                inst_count!(self, "fcvt.s.lu");

                                self.fregs
                                    .write(rd, ((self.xregs.read(rs1) as u64) as f32) as f64);
                            }
                            _ => {
                                return Err(Exception::IllegalInstruction);
                            }
                        }
                    }
                    0x69 => {
                        match rs2 {
                            0x0 => {
                                // fcvt.d.w
                                inst_count!(self, "fcvt.d.w");

                                self.fregs.write(rd, (self.xregs.read(rs1) as i32) as f64);
                            }
                            0x1 => {
                                // fcvt.d.wu
                                inst_count!(self, "fcvt.d.wu");

                                self.fregs.write(rd, (self.xregs.read(rs1) as u32) as f64);
                            }
                            0x2 => {
                                // fcvt.d.l
                                inst_count!(self, "fcvt.d.l");

                                self.fregs.write(rd, self.xregs.read(rs1) as f64);
                            }
                            0x3 => {
                                // fcvt.d.lu
                                inst_count!(self, "fcvt.d.lu");

                                self.fregs.write(rd, self.xregs.read(rs1) as f64);
                            }
                            _ => {
                                return Err(Exception::IllegalInstruction);
                            }
                        }
                    }
                    0x70 => {
                        match funct3 {
                            0x0 => {
                                // fmv.x.w
                                inst_count!(self, "fmv.x.w");

                                // "The bits are not modified in the transfer"
                                self.xregs.write(
                                    rd,
                                    (self.fregs.read(rs1).to_bits() as u32) as i32 as i64 as u64,
                                );
                            }
                            0x1 => {
                                // fclass.s
                                inst_count!(self, "fclass.s");

                                let f = self.fregs.read(rs1);
                                match f.classify() {
                                    FpCategory::Infinite => {
                                        self.xregs
                                            .write(rd, if f.is_sign_negative() { 0 } else { 7 });
                                    }
                                    FpCategory::Normal => {
                                        self.xregs
                                            .write(rd, if f.is_sign_negative() { 1 } else { 6 });
                                    }
                                    FpCategory::Subnormal => {
                                        self.xregs
                                            .write(rd, if f.is_sign_negative() { 2 } else { 5 });
                                    }
                                    FpCategory::Zero => {
                                        self.xregs
                                            .write(rd, if f.is_sign_negative() { 3 } else { 4 });
                                    }
                                    // don't support a signaling nan, only support a quiet nan.
                                    FpCategory::Nan => self.xregs.write(rd, 9),
                                }
                            }
                            _ => {
                                return Err(Exception::IllegalInstruction);
                            }
                        }
                    }
                    0x71 => {
                        match funct3 {
                            0x0 => {
                                // fmv.x.d
                                inst_count!(self, "fmv.x.d");

                                // "FMV.X.D and FMV.D.X do not modify the bits being transferred"
                                self.xregs.write(rd, self.fregs.read(rs1).to_bits());
                            }
                            0x1 => {
                                // fclass.d
                                inst_count!(self, "fclass.d");

                                let f = self.fregs.read(rs1);
                                match f.classify() {
                                    FpCategory::Infinite => {
                                        self.xregs
                                            .write(rd, if f.is_sign_negative() { 0 } else { 7 });
                                    }
                                    FpCategory::Normal => {
                                        self.xregs
                                            .write(rd, if f.is_sign_negative() { 1 } else { 6 });
                                    }
                                    FpCategory::Subnormal => {
                                        self.xregs
                                            .write(rd, if f.is_sign_negative() { 2 } else { 5 });
                                    }
                                    FpCategory::Zero => {
                                        self.xregs
                                            .write(rd, if f.is_sign_negative() { 3 } else { 4 });
                                    }
                                    // don't support a signaling nan, only support a quiet nan.
                                    FpCategory::Nan => self.xregs.write(rd, 9),
                                }
                            }
                            _ => {
                                return Err(Exception::IllegalInstruction);
                            }
                        }
                    }
                    0x78 => {
                        // fmv.w.x
                        inst_count!(self, "fmv.w.x");

                        // "The bits are not modified in the transfer"
                        self.fregs
                            .write(rd, f32::from_bits(self.xregs.read(rs1) as u32) as f64);
                    }
                    0x79 => {
                        // fmv.d.x
                        inst_count!(self, "fmv.d.x");

                        // "FMV.X.D and FMV.D.X do not modify the bits being transferred"
                        self.fregs.write(rd, f64::from_bits(self.xregs.read(rs1)));
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction);
                    }
                }
            }
            0x63 => {
                // RV32I
                // imm[12|10:5|4:1|11] = inst[31|30:25|11:8|7]
                let imm = (((inst & 0x80000000) as i32 as i64 >> 19) as u64)
                    | ((inst & 0x80) << 4) // imm[11]
                    | ((inst >> 20) & 0x7e0) // imm[10:5]
                    | ((inst >> 7) & 0x1e); // imm[4:1]

                match funct3 {
                    0x0 => {
                        // beq
                        inst_count!(self, "beq");

                        if self.xregs.read(rs1) == self.xregs.read(rs2) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x1 => {
                        // bne
                        inst_count!(self, "bne");

                        if self.xregs.read(rs1) != self.xregs.read(rs2) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x4 => {
                        // blt
                        inst_count!(self, "blt");

                        if (self.xregs.read(rs1) as i64) < (self.xregs.read(rs2) as i64) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x5 => {
                        // bge
                        inst_count!(self, "bge");

                        if (self.xregs.read(rs1) as i64) >= (self.xregs.read(rs2) as i64) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x6 => {
                        // bltu
                        inst_count!(self, "bltu");

                        if self.xregs.read(rs1) < self.xregs.read(rs2) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x7 => {
                        // bgeu
                        inst_count!(self, "bgeu");

                        if self.xregs.read(rs1) >= self.xregs.read(rs2) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction);
                    }
                }
            }
            0x67 => {
                // jalr
                inst_count!(self, "jalr");

                // Don't add 4 because the pc already moved on.
                let t = self.pc;

                let offset = (inst as i32 as i64) >> 20;
                let target = ((self.xregs.read(rs1) as i64).wrapping_add(offset)) & !1;

                self.pc = target as u64;
                self.xregs.write(rd, t);
            }
            0x6F => {
                // jal
                inst_count!(self, "jal");

                self.xregs.write(rd, self.pc);

                // imm[20|10:1|11|19:12] = inst[31|30:21|20|19:12]
                let offset = (((inst & 0x80000000) as i32 as i64 >> 11) as u64) // imm[20]
                    | (inst & 0xff000) // imm[19:12]
                    | ((inst >> 9) & 0x800) // imm[11]
                    | ((inst >> 20) & 0x7fe); // imm[10:1]

                self.pc = self.pc.wrapping_add(offset).wrapping_sub(4);
            }
            0x73 => {
                // RV32I, RVZicsr, and supervisor ISA
                let csr_addr = ((inst >> 20) & 0xfff) as u16;
                match funct3 {
                    0x0 => {
                        match (rs2, funct7) {
                            (0x0, 0x0) => {
                                // ecall
                                inst_count!(self, "ecall");

                                // Makes a request of the execution environment by raising an
                                // environment call exception.
                                match self.mode {
                                    Mode::User => {
                                        return Err(Exception::EnvironmentCallFromUMode);
                                    }
                                    Mode::Supervisor => {
                                        return Err(Exception::EnvironmentCallFromSMode);
                                    }
                                    Mode::Machine => {
                                        return Err(Exception::EnvironmentCallFromMMode);
                                    }
                                    _ => {
                                        return Err(Exception::IllegalInstruction);
                                    }
                                }
                            }
                            (0x1, 0x0) => {
                                // ebreak
                                inst_count!(self, "ebreak");

                                // Makes a request of the debugger bu raising a Breakpoint
                                // exception.
                                return Err(Exception::Breakpoint);
                            }
                            (0x2, 0x0) => {
                                // uret
                                inst_count!(self, "uret");
                                panic!("uret: not implemented yet. pc {}", self.pc);
                            }
                            (0x2, 0x8) => {
                                // sret
                                inst_count!(self, "sret");

                                // "The RISC-V Reader" book says:
                                // "Returns from a supervisor-mode exception handler. Sets the pc to
                                // CSRs[sepc], the privilege mode to CSRs[sstatus].SPP,
                                // CSRs[sstatus].SIE to CSRs[sstatus].SPIE, CSRs[sstatus].SPIE to
                                // 1, and CSRs[sstatus].SPP to 0.", but the implementation in QEMU
                                // and Spike use `mstatus` instead of `sstatus`.

                                // Set the program coutner to the supervisor exception program
                                // counter (SEPC).
                                self.pc = self.state.read(SEPC);

                                // TODO: Check TSR field

                                // Set the current privileged mode depending on a privious
                                // privilege mode for supervisor mode (SPP, 8).
                                self.mode = match self.state.read_bit(SSTATUS, 8) {
                                    0 => Mode::User,
                                    1 => Mode::Supervisor,
                                    _ => Mode::Debug,
                                };
                                // Read a privious interrupt-enable bit for supervisor mode (SPIE,
                                // 5), and set a global interrupt-enable bit for supervisor mode
                                // (SIE, 1) to it.
                                self.state
                                    .write_bit(SSTATUS, 1, self.state.read_bit(SSTATUS, 5));

                                // Set a privious interrupt-enable bit for supervisor mode (SPIE,
                                // 5) to 1.
                                self.state.write_bit(SSTATUS, 5, 1);
                                // Set a privious privilege mode for supervisor mode (SPP, 8) to 0.
                                self.state.write_bit(SSTATUS, 8, 0);
                            }
                            (0x2, 0x18) => {
                                // mret
                                inst_count!(self, "mret");

                                // "The RISC-V Reader" book says:
                                // "Returns from a machine-mode exception handler. Sets the pc to
                                // CSRs[mepc], the privilege mode to CSRs[mstatus].MPP,
                                // CSRs[mstatus].MIE to CSRs[mstatus].MPIE, and CSRs[mstatus].MPIE
                                // to 1; and, if user mode is supported, sets CSRs[mstatus].MPP to
                                // 0".

                                // Set the program coutner to the machine exception program
                                // counter (MEPC).
                                self.pc = self.state.read(MEPC);

                                // Set the current privileged mode depending on a privious
                                // privilege mode for machine  mode (MPP, 11..13).
                                self.mode = match self.state.read_bits(MSTATUS, 11..13) {
                                    0b00 => Mode::User,
                                    0b01 => Mode::Supervisor,
                                    0b11 => Mode::Machine,
                                    _ => Mode::Debug,
                                };

                                // Read a privious interrupt-enable bit for machine mode (MPIE, 7),
                                // and set a global interrupt-enable bit for machine mode (MIE, 3)
                                // to it.
                                self.state
                                    .write_bit(MSTATUS, 3, self.state.read_bit(MSTATUS, 7));

                                // Set a privious interrupt-enable bit for machine mode (MPIE, 7)
                                // to 1.
                                self.state.write_bit(MSTATUS, 7, 1);

                                // Set a privious privilege mode for machine mode (MPP, 11..13) to
                                // 0.
                                self.state.write_bits(MSTATUS, 11..13, 0b00);
                            }
                            (0x5, 0x8) => {
                                // wfi
                                inst_count!(self, "wfi");
                                // "provides a hint to the implementation that the current
                                // hart can be stalled until an interrupt might need servicing."
                                self.idle = true;
                            }
                            (_, 0x9) => {
                                // sfence.vma
                                inst_count!(self, "sfence.vma");
                                // "SFENCE.VMA is used to synchronize updates to in-memory
                                // memory-management data structures with current execution"
                            }
                            (_, 0x11) => {
                                // hfence.bvma
                                inst_count!(self, "hfence.bvma");
                            }
                            (_, 0x51) => {
                                // hfence.gvma
                                inst_count!(self, "hfence.gvma");
                            }
                            _ => {
                                return Err(Exception::IllegalInstruction);
                            }
                        }
                    }
                    0x1 => {
                        // csrrw
                        inst_count!(self, "csrrw");

                        let t = self.state.read(csr_addr);
                        self.state.write(csr_addr, self.xregs.read(rs1));
                        self.xregs.write(rd, t);

                        if csr_addr == SATP {
                            self.update_paging();
                        }
                    }
                    0x2 => {
                        // csrrs
                        inst_count!(self, "csrrs");

                        let t = self.state.read(csr_addr);
                        self.state.write(csr_addr, t | self.xregs.read(rs1));
                        self.xregs.write(rd, t);

                        if csr_addr == SATP {
                            self.update_paging();
                        }
                    }
                    0x3 => {
                        // csrrc
                        inst_count!(self, "csrrc");

                        let t = self.state.read(csr_addr);
                        self.state.write(csr_addr, t & (!self.xregs.read(rs1)));
                        self.xregs.write(rd, t);

                        if csr_addr == SATP {
                            self.update_paging();
                        }
                    }
                    0x5 => {
                        // csrrwi
                        inst_count!(self, "csrrwi");

                        let zimm = rs1;
                        self.xregs.write(rd, self.state.read(csr_addr));
                        self.state.write(csr_addr, zimm);

                        if csr_addr == SATP {
                            self.update_paging();
                        }
                    }
                    0x6 => {
                        // csrrsi
                        inst_count!(self, "csrrsi");

                        let zimm = rs1;
                        let t = self.state.read(csr_addr);
                        self.state.write(csr_addr, t | zimm);
                        self.xregs.write(rd, t);

                        if csr_addr == SATP {
                            self.update_paging();
                        }
                    }
                    0x7 => {
                        // csrrci
                        inst_count!(self, "csrrci");

                        let zimm = rs1;
                        let t = self.state.read(csr_addr);
                        self.state.write(csr_addr, t & (!zimm));
                        self.xregs.write(rd, t);

                        if csr_addr == SATP {
                            self.update_paging();
                        }
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction);
                    }
                }
            }
            _ => {
                return Err(Exception::IllegalInstruction);
            }
        }
        Ok(inst)
    }
}
