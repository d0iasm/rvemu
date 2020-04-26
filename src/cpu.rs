//! The cpu module contains the privileged mode, registers, and CPU.

pub const REGISTERS_COUNT: usize = 32;

use std::cmp;
use std::cmp::PartialEq;
use std::fmt;
use std::num::FpCategory;

use crate::{
    bus::{Bus, DRAM_BASE},
    csr::*,
    devices::{
        uart::UART_IRQ,
        virtio::{Virtio, VIRTIO_IRQ},
    },
    exception::Exception,
    interrupt::Interrupt,
    memory::MEMORY_SIZE,
};

/// The stack pointer.
const SP: u64 = 2;

/// The page size (4 KiB) for the virtual memory system.
const PAGE_SIZE: u64 = 4096;

/// The privileged mode.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Mode {
    User = 0b00,
    Supervisor = 0b01,
    Machine = 0b11,
    Debug,
}

impl Mode {
    /// Check that the current privilege mode meets the required mode.
    pub fn require(&self, require: Mode) -> Result<(), Exception> {
        match require {
            Mode::User => {
                if self == &Mode::Machine || self == &Mode::Supervisor || self == &Mode::User {
                    return Ok(());
                }
                Err(Exception::IllegalInstruction)
            }
            Mode::Supervisor => {
                if self == &Mode::Machine || self == &Mode::Supervisor {
                    return Ok(());
                }
                Err(Exception::IllegalInstruction)
            }
            Mode::Machine => {
                if self == &Mode::Machine {
                    return Ok(());
                }
                Err(Exception::IllegalInstruction)
            }
            _ => Err(Exception::IllegalInstruction),
        }
    }
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
        xregs[SP as usize] = MEMORY_SIZE + DRAM_BASE;
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
        let mut output = String::from("");
        for i in (0..REGISTERS_COUNT).step_by(4) {
            output = format!(
                "{}\n{}",
                output,
                format!(
                    "x{:02}={:>#18x} x{:02}={:>#18x} x{:02}={:>#18x} x{:02}={:>#18x}",
                    i,
                    self.read(i as u64),
                    i + 1,
                    self.read(i as u64 + 1),
                    i + 2,
                    self.read(i as u64 + 2),
                    i + 3,
                    self.read(i as u64 + 3)
                )
            );
        }
        // Remove the first new line.
        output.remove(0);
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
        let mut output = String::from("");
        for i in (0..REGISTERS_COUNT).step_by(4) {
            output = format!(
                "{}\n{}",
                output,
                format!(
                    "f{:02}={:>width$.prec$} f{:02}={:>width$.prec$} f{:02}={:>width$.prec$} f{:02}={:>width$.prec$}",
                    i,
                    self.read(i as u64),
                    i + 1,
                    self.read(i as u64 + 1),
                    i + 2,
                    self.read(i  as u64+ 2),
                    i + 3,
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
    pub xregs: XRegisters,
    pub fregs: FRegisters,
    pub pc: u64,
    pub state: State,
    pub mode: Mode,
    pub bus: Bus,
    pub enable_paging: bool,
    pub page_table: u64,
    pub debug: bool,
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
            bus: Bus::new(),
            enable_paging: false,
            page_table: 0,
            debug: false,
        }
    }

    /// Reset CPU states.
    pub fn reset(&mut self) {
        self.pc = 0;
        self.state.reset();
        // TODO: reset CPU mode to machine or not?
        for i in 0..REGISTERS_COUNT {
            self.xregs.write(i as u64, 0);
            self.fregs.write(i as u64, 0.0);
        }
    }

    /// Increment the timer register (mtimer) in CLINT.
    pub fn timer_increment(&mut self) {
        self.bus.clint.increment();
    }

    /// Check interrupt flags for all devices that can interrupt.
    pub fn check_interrupt(&mut self) -> Option<Interrupt> {
        // Check if an interrupt register is enable. If it's disable, no interrupt occurs.
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
            Mode::User => {
                // Check if the UIE bit is enabled.
                if self.state.read(USTATUS) & 1 == 0 {
                    return None;
                }
            }
            _ => {}
        }

        // Software interrupt for timer (CLINT).
        // TODO: Actually, the timer interrupt caused when the MTIP bit in MIP is set, but it's not
        // used for simplicity.
        if self.bus.clint.is_interrupting() {
            match self.mode {
                Mode::Machine => return Some(Interrupt::MachineSoftwareInterrupt),
                Mode::Supervisor => return Some(Interrupt::SupervisorSoftwareInterrupt),
                Mode::User => return Some(Interrupt::UserSoftwareInterrupt),
                _ => return Some(Interrupt::MachineSoftwareInterrupt),
            }
        }

        // External interrupt for UART and virtio.
        let irq;
        if self.bus.uart.is_interrupting() {
            irq = UART_IRQ;
        } else if self.bus.virtio.is_interrupting() {
            // Access disk by direct memory access (DMA). An interrupt is raised after a disk
            // access is done.
            Virtio::disk_access(self);
            irq = VIRTIO_IRQ;
        } else {
            return None;
        }

        match self.mode {
            Mode::Machine => Some(Interrupt::MachineExternalInterrupt(irq)),
            Mode::Supervisor => Some(Interrupt::SupervisorExternalInterrupt(irq)),
            Mode::User => Some(Interrupt::UserExternalInterrupt(irq)),
            _ => Some(Interrupt::MachineExternalInterrupt(irq)),
        }
    }

    /// Translate a virtual address to a physical address for the paged virtual-memory system.
    pub fn translate(&mut self, addr: u64) -> Result<u64, Exception> {
        if !self.enable_paging {
            return Ok(addr);
        }

        // TODO: Support only Sv39

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
            pte = self.bus.read64(a + vpn[i as usize] * 8)?;
            // 3. If pte.v = 0, or if pte.r = 0 and pte.w = 1, stop and raise a page-fault
            //    exception corresponding to the original access type.
            let v = pte & 1;
            let r = (pte >> 1) & 1;
            let w = (pte >> 2) & 1;
            let x = (pte >> 3) & 1;
            if v == 0 || (r == 0 && w == 1) {
                // TODO: raise InstructionPageFault, LoadPageFault, or StoreAMOPageFault
                // depending on the original access type.
                return Err(Exception::InstructionPageFault);
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
                // TODO: raise InstructionPageFault, LoadPageFault, or StoreAMOPageFault
                // depending on the original access type.
                return Err(Exception::InstructionPageFault);
            }
        }
        // TODO: implement step 5
        // 5. A leaf PTE has been found. Determine if the requested memory access is
        //    allowed by the pte.r, pte.w, pte.x, and pte.u bits, given the current
        //    privilege mode and the value of the SUM and MXR fields of the mstatus
        //    register. If not, stop and raise a page-fault exception corresponding
        //    to the original access type.

        // TODO: implement step 6
        // 6. If i > 0 and pte.ppn[i−1:0] != 0, this is a misaligned superpage; stop and
        //    raise a page-fault exception corresponding to the original access type.

        // TODO: implement step 7
        // 7. If pte.a = 0, or if the memory access is a store and pte.d = 0, either raise
        //    a page-fault exception corresponding to the original access type, or:
        //    • Set pte.a to 1 and, if the memory access is a store, also set pte.d to 1.
        //    • If this access violates a PMA or PMP check, raise an access exception
        //    corresponding to the original access type.
        //    • This update and the loading of pte in step 2 must be atomic; in particular,
        //    no intervening store to the PTE may be perceived to have occurred in-between.

        // 8. The translation is successful. The translated physical address is given as
        //    follows:
        //    • pa.pgoff = va.pgoff.
        //    • If i > 0, then this is a superpage translation and pa.ppn[i−1:0] =
        //    va.vpn[i−1:0].
        //    • pa.ppn[LEVELS−1:i] = pte.ppn[LEVELS−1:i].
        let offset = addr & 0xfff;
        return Ok(if i > 0 {
            let ppn = [
                (pte >> 10) & 0x1ff,
                (pte >> 19) & 0x1ff,
                (pte >> 28) & 0x03ff_ffff,
            ];
            (ppn[2] << 30) | (ppn[1] << 21) | (vpn[0] << 12) | offset
        } else {
            let ppn = (pte >> 10) & 0x0fff_ffff_ffff;
            (ppn << 12) | offset
        });
    }

    /// Read a byte from the system bus with the translation a virtual address to a physical address
    /// if it is enabled.
    pub fn read8(&mut self, v_addr: u64) -> Result<u64, Exception> {
        let p_addr = self.translate(v_addr)?;
        self.bus.read8(p_addr)
    }

    /// Read 2 bytes from the system bus with the translation a virtual address to a physical address
    /// if it is enabled.
    pub fn read16(&mut self, v_addr: u64) -> Result<u64, Exception> {
        let p_addr = self.translate(v_addr)?;
        self.bus.read16(p_addr)
    }

    /// Read 4 bytes from the system bus with the translation a virtual address to a physical address
    /// if it is enabled.
    pub fn read32(&mut self, v_addr: u64) -> Result<u64, Exception> {
        let p_addr = self.translate(v_addr)?;
        self.bus.read32(p_addr)
    }

    /// Read 8 bytes from the system bus with the translation a virtual address to a physical address
    /// if it is enabled.
    pub fn read64(&mut self, v_addr: u64) -> Result<u64, Exception> {
        let p_addr = self.translate(v_addr)?;
        self.bus.read64(p_addr)
    }

    /// Write a byte to the system bus with the translation a virtual address to a physical address
    /// if it is enabled.
    pub fn write8(&mut self, v_addr: u64, val: u64) -> Result<(), Exception> {
        let p_addr = self.translate(v_addr)?;
        self.bus.write8(p_addr, val)
    }

    /// Write 2 bytes to the system bus with the translation a virtual address to a physical address
    /// if it is enabled.
    pub fn write16(&mut self, v_addr: u64, val: u64) -> Result<(), Exception> {
        let p_addr = self.translate(v_addr)?;
        self.bus.write16(p_addr, val)
    }

    /// Write 4 bytes to the system bus with the translation a virtual address to a physical address
    /// if it is enabled.
    pub fn write32(&mut self, v_addr: u64, val: u64) -> Result<(), Exception> {
        let p_addr = self.translate(v_addr)?;
        self.bus.write32(p_addr, val)
    }

    /// Write 8 bytes to the system bus with the translation a virtual address to a physical address
    /// if it is enabled.
    pub fn write64(&mut self, v_addr: u64, val: u64) -> Result<(), Exception> {
        let p_addr = self.translate(v_addr)?;
        self.bus.write64(p_addr, val)
    }

    /// Fetch the next instruction from the memory at the current program counter.
    pub fn fetch32(&mut self) -> Result<u64, Exception> {
        self.read32(self.pc)
    }

    /// Fetch the next instruction from the memory at the current program counter.
    pub fn fetch16(&mut self) -> Result<u64, Exception> {
        self.read16(self.pc)
    }

    /// Execute an instruction. Raises an exception if something is wrong, otherwise, returns
    /// nothing.
    pub fn tick(&mut self) -> Result<(), Exception> {
        // Fetch.
        let inst16 = self.fetch16()?;
        match inst16 & 0b11 {
            0 | 1 | 2 => {
                if inst16 == 0 {
                    // Unimplemented instruction, since all bits are 0.
                    return Err(Exception::IllegalInstruction);
                }
                self.tick_c()?
            }
            _ => self.tick_g()?,
        }
        Ok(())
    }

    /// Execute a compressed instruction. Raised an exception if something is wrong, otherwise,
    /// returns nothing. It also increments the program counter by 2 bytes.
    pub fn tick_c(&mut self) -> Result<(), Exception> {
        // 1. Fetch.
        let inst = self.fetch16()?;

        // Add 2 bytes to the program counter.
        self.pc += 2;

        // 2. Decode.
        let opcode = inst & 0b11;
        let funct3 = (inst >> 13) & 0b111;
        //let funct4 = (inst & 0xf000) >> 12;
        //let funct6 = (inst & 0xfc00) >> 10;

        // 3. Execute.
        match opcode {
            0 => {
                // Quadrant 0.
                // Compressed instructions have 3-bit field for popular registers,
                // which correspond to registers x8 to x15.
                let rd_rs2_short = ((inst >> 2) & 0b111) + 8;
                let rs1_short = ((inst >> 7) & 0b111) + 8;

                match funct3 {
                    0x0 => {
                        // c.addi4spn
                        // nzuimm[5:4|9:6|2|3] = inst[12:11|10:7|6|5]
                        let mut nzuimm = ((inst >> 1) & 0x3c0) // imm[9:6]
                            | ((inst >> 7) & 0x30) // imm[5:4]
                            | ((inst >> 2) & 0x8) // imm[3]
                            | ((inst >> 4) & 0x4); // imm[2]
                        nzuimm = match (inst & 0x3c0) == 0 {
                            true => nzuimm,
                            // Sign-extended.
                            false => (0xfc00 | nzuimm) as i16 as i64 as u64,
                        };
                        if nzuimm == 0 {
                            return Err(Exception::IllegalInstruction);
                        }
                        self.xregs
                            .write(rd_rs2_short, self.xregs.read(2).wrapping_add(nzuimm));
                    }
                    0x1 => {
                        // c.fld
                        // uimm[5:3|7:6] = isnt[12:10|6:5]
                        let uimm = (((inst << 1) & 0xc0) as i8 as i64 as u64) // imm[7:6]
                            | ((inst >> 7) & 0x38); // imm[5:3]
                        let val =
                            f64::from_bits(self.read64(self.xregs.read(2).wrapping_add(uimm))?);
                        self.fregs.write(rd_rs2_short, val);
                    }
                    0x2 => {
                        // c.lw
                        // uimm[5:3|2|6] = isnt[12:10|6|5]
                        let uimm = ((inst << 1) & 0x40) // imm[6]
                            | ((inst >> 7) & 0x38) // imm[5:3]
                            | ((inst >> 4) & 0x4); // imm[2]
                        let val = self.read32(self.xregs.read(rs1_short).wrapping_add(uimm))?;
                        self.xregs.write(rd_rs2_short, val as i32 as i64 as u64);
                    }
                    0x3 => {
                        // c.ld
                        // uimm[5:3|7:6] = isnt[12:10|6:5]
                        let uimm = (((inst << 1) & 0xc0) as i8 as i64 as u64) // imm[7:6]
                            | ((inst >> 7) & 0x38); // imm[5:3]
                        let val = self.read64(self.xregs.read(rs1_short).wrapping_add(uimm))?;
                        self.xregs.write(rd_rs2_short, val);
                    }
                    0x4 => {
                        // Reserved.
                    }
                    0x5 => {
                        // c.fsd
                        // uimm[5:3|7:6] = isnt[12:10|6:5]
                        let uimm = (((inst << 1) & 0xc0) as i8 as i64 as u64) // imm[7:6]
                            | ((inst >> 7) & 0x38); // imm[5:3]
                        let addr = self.xregs.read(rs1_short).wrapping_add(uimm);
                        self.write64(addr, self.fregs.read(rd_rs2_short).to_bits() as u64)?;
                    }
                    0x6 => {
                        // c.sw
                        // uimm[5:3|2|6] = isnt[12:10|6|5]
                        let uimm = ((inst << 1) & 0x40) // imm[6]
                            | ((inst >> 7) & 0x38) // imm[5:3]
                            | ((inst >> 4) & 0x4); // imm[2]
                        let addr = self.xregs.read(rs1_short).wrapping_add(uimm);
                        self.write32(addr, self.xregs.read(rd_rs2_short))?;
                    }
                    0x7 => {
                        // c.sd
                        // uimm[5:3|7:6] = isnt[12:10|6:5]
                        let uimm = (((inst << 1) & 0xc0) as i8 as i64 as u64) // imm[7:6]
                            | ((inst >> 7) & 0x38); // imm[5:3]
                        let addr = self.xregs.read(rs1_short).wrapping_add(uimm);
                        self.write64(addr, self.xregs.read(rd_rs2_short))?;
                    }
                    _ => {}
                }
            }
            1 => {
                // Quadrant 1.
                let rd_rs1 = (inst >> 7) & 0x1f;
                // imm[5|4:0] = inst[12|6:2]
                let mut imm = ((inst >> 7) & 0x20) | ((inst >> 2) & 0x1f);
                // Sign-extended.
                imm = match (imm & 0x20) == 0 {
                    true => imm,
                    false => (0xf0 | imm) as i8 as i64 as u64,
                };
                match funct3 {
                    0x0 => {
                        // c.addi
                        self.xregs
                            .write(rd_rs1, self.xregs.read(rd_rs1).wrapping_add(imm));
                    }
                    0x1 => {
                        // c.addiw
                        self.xregs.write(
                            rd_rs1,
                            self.xregs.read(rd_rs1).wrapping_add(imm) as i32 as i64 as u64,
                        );
                    }
                    0x2 => {
                        // c.li
                        self.xregs.write(rd_rs1, imm as i32 as i64 as u64);
                    }
                    0x3 => {
                        match rd_rs1 {
                            0 => {}
                            2 => {
                                // c.addi16sp
                                // nzimm[9|4|6|8:7|5] = inst[12|6|5|4:3|2]
                                let mut imm = ((inst >> 3) & 0x200) // nzimm[9]
                                    | ((inst >> 2) & 0x10) // nzimm[4]
                                    | ((inst << 1) & 0x40) // nzimm[6]
                                    | ((inst << 4) & 0x180) // nzimm[8:7]
                                    | ((inst << 3) & 0x20); // nzimm[5]
                                                            // Sign-extended.
                                imm = match (imm & 0x200) == 0 {
                                    true => imm,
                                    false => (0xfe00 | imm) as i16 as i32 as i64 as u64,
                                };
                                self.xregs.write(2, self.xregs.read(2).wrapping_add(imm));
                            }
                            _ => {
                                // c.lui
                                // nzimm[17|16:12] = inst[12|6:2]
                                let mut imm = ((inst << 5) & 0x20000) | ((inst << 10) & 0x1f000);
                                // Sign-extended.
                                imm = match (imm & 0x20000) == 0 {
                                    true => imm,
                                    false => (0xfffe0000 | imm) as i32 as i64 as u64,
                                };
                                self.xregs.write(rd_rs1, imm);
                            }
                        }
                    }
                    0x4 => {
                        let rd_rs1_dash = rd_rs1 & 0b111;
                        let rd = rd_rs1_dash + 8;
                        match (rd_rs1 >> 3) & 0b11 {
                            0x0 => {
                                // c.srli
                                self.xregs
                                    .write(rd, self.xregs.read(rd).wrapping_shr(imm as u32));
                            }
                            0x1 => {
                                // c.srai
                                self.xregs.write(
                                    rd,
                                    (self.xregs.read(rd) as i64).wrapping_shr(imm as u32) as u64,
                                );
                            }
                            0x2 => {
                                // c.andi
                                self.xregs.write(rd, self.xregs.read(rd) & imm);
                            }
                            0x3 => {
                                let rs2 = (imm & 0b111) + 8;
                                match ((imm >> 5) & 0b1, (imm >> 3) & 0b11) {
                                    (0x0, 0x0) => {
                                        // c.sub
                                        self.xregs
                                            .write(rd, self.xregs.read(rd) - self.xregs.read(rs2));
                                    }
                                    (0x0, 0x1) => {
                                        // c.xor
                                        self.xregs
                                            .write(rd, self.xregs.read(rd) ^ self.xregs.read(rs2));
                                    }
                                    (0x0, 0x2) => {
                                        // c.or
                                        self.xregs
                                            .write(rd, self.xregs.read(rd) | self.xregs.read(rs2));
                                    }
                                    (0x0, 0x3) => {
                                        // c.and
                                        self.xregs
                                            .write(rd, self.xregs.read(rd) & self.xregs.read(rs2));
                                    }
                                    (0x1, 0x0) => {
                                        // c.subw
                                        self.xregs.write(
                                            rd,
                                            ((self
                                                .xregs
                                                .read(rd)
                                                .wrapping_sub(self.xregs.read(rs2)))
                                                as i32)
                                                as u64,
                                        );
                                    }
                                    (0x1, 0x1) => {
                                        // c.addw
                                        self.xregs.write(
                                            rd,
                                            self.xregs.read(rd).wrapping_add(self.xregs.read(rs2))
                                                as i32
                                                as i64
                                                as u64,
                                        );
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    0x5 => {
                        // c.j
                        // imm[11|4|9:8|10|6|7|3:1|5] = inst[12|11|10:9|8|7|6|5:3|2]
                        let imm11 = (((inst & 0x1000) as i32) as i64) >> 12;
                        let imm10 = (inst >> 8) & 0b1;
                        let imm9_8 = (inst >> 9) & 0b11;
                        let imm7 = (inst >> 6) & 0b1;
                        let imm6 = (inst >> 7) & 0b1;
                        let imm5 = (inst >> 2) & 0b1;
                        let imm4 = (inst >> 11) & 0b1;
                        let imm3_1 = (inst >> 3) & 0b111;
                        let offset = (imm11 << 11) as u64
                            | imm10 << 10
                            | imm9_8 << 8
                            | imm7 << 7
                            | imm6 << 6
                            | imm5 << 5
                            | imm4 << 4
                            | imm3_1 << 1;
                        self.pc += offset;
                    }
                    0x6 => {
                        // c.beqz
                        let rd_rs1_dash = rd_rs1 & 0b111;
                        let rs1 = rd_rs1_dash + 8;
                        // imm[8|4:3|7:6|2:1|5] = inst[12|11:10|6:5|4:3|2]
                        let imm8 = (((inst & 0x1000) as i32) as i64) >> 12;
                        let imm7_6 = (inst >> 5) & 0b11;
                        let imm5 = (inst >> 2) & 0b1;
                        let imm4_3 = (inst >> 10) & 0b11;
                        let imm2_1 = (inst >> 3) & 0b11;
                        let imm = (imm8 << 8) as u64
                            | imm7_6 << 6
                            | imm5 << 5
                            | imm4_3 << 3
                            | imm2_1 << 1;
                        if self.xregs.read(rs1) == 0 {
                            self.pc += imm;
                        }
                    }
                    0x7 => {
                        // c.bnez
                        let rd_rs1_dash = rd_rs1 & 0b111;
                        let rs1 = rd_rs1_dash + 8;
                        // imm[8|4:3|7:6|2:1|5] = inst[12|11:10|6:5|4:3|2]
                        let imm8 = (((inst & 0x1000) as i32) as i64) >> 12;
                        let imm7_6 = (inst >> 5) & 0b11;
                        let imm5 = (inst >> 2) & 0b1;
                        let imm4_3 = (inst >> 10) & 0b11;
                        let imm2_1 = (inst >> 3) & 0b11;
                        let imm = (imm8 << 8) as u64
                            | imm7_6 << 6
                            | imm5 << 5
                            | imm4_3 << 3
                            | imm2_1 << 1;
                        if self.xregs.read(rs1) != 0 {
                            self.pc += imm;
                        }
                    }
                    _ => {}
                }
            }
            2 => {
                // C2
                let rd_rs1 = (inst >> 7) & 0x1f;
                match funct3 {
                    0x0 => {
                        // c.slli
                        // imm[5|4:0] = inst[12|6:2]
                        let imm5 = (((inst & 0x1000) as i32) as i64) >> 12;
                        let imm4_0 = (inst >> 2) & 0b1111;
                        let nzuimm = (imm5 << 5) as u64 | imm4_0;
                        self.xregs.write(rd_rs1, self.xregs.read(rd_rs1) << nzuimm);
                    }
                    0x1 => {
                        // c.fldsp
                        // imm[5|4:3|8:6] = inst[12|6:5|4:2]
                        let imm8_6 = (((inst & 0x10) as i32) as i64) >> 2;
                        let imm5 = (inst >> 12) & 0b1;
                        let imm4_3 = (inst >> 5) & 0b11;
                        let uimm = (imm8_6 << 6) as u64 | imm5 << 5 | imm4_3 << 3;
                        let val = f64::from_bits(self.read64(self.xregs.read(rd_rs1) + uimm)?);
                        self.fregs.write(rd_rs1, val);
                    }
                    0x2 => {
                        // c.lwsp
                        // imm[5|4:2|7:6] = inst[12|6:4|3:2]
                        let imm7_6 = (((inst & 0x8) as i32) as i64) >> 2;
                        let imm5 = (inst >> 12) & 0b1;
                        let imm4_2 = (inst >> 4) & 0b111;
                        let uimm = (imm7_6 << 6) as u64 | imm5 << 5 | imm4_2 << 2;
                        let val = self.read32(self.xregs.read(2).wrapping_add(uimm))?;
                        self.xregs.write(rd_rs1, val as i32 as i64 as u64);
                    }
                    0x3 => {
                        // c.ldsp
                        // imm[5|4:3|8:6] = inst[12|6:5|4:2]
                        let imm8_6 = (((inst & 0x10) as i32) as i64) >> 2;
                        let imm5 = (inst >> 12) & 0b1;
                        let imm4_3 = (inst >> 5) & 0b11;
                        let uimm = (imm8_6 << 6) as u64 | imm5 << 5 | imm4_3 << 3;
                        let val = self.read64(self.xregs.read(2).wrapping_add(uimm))?;
                        self.xregs.write(rd_rs1, val);
                    }
                    0x4 => {
                        let rs2 = (inst >> 2) & 0x1f;
                        match ((inst & 0x1000) == 0, rs2 == 0) {
                            (true, true) => {
                                // c.jr
                                self.pc = self.xregs.read(rd_rs1);
                            }
                            (true, false) => {
                                // c.mv
                                self.xregs.write(rd_rs1, self.xregs.read(rs2));
                            }
                            (false, true) => {
                                if rd_rs1 == 0 {
                                    // c.ebreak
                                    return Err(Exception::Breakpoint);
                                } else {
                                    // c.jalr
                                    let t = self.pc + 2;
                                    self.pc = self.xregs.read(rd_rs1);
                                    self.xregs.write(1, t);
                                }
                            }
                            (false, false) => {
                                // c.add
                                self.xregs
                                    .write(rd_rs1, self.xregs.read(rd_rs1) + self.xregs.read(rs2));
                            }
                        }
                    }
                    0x5 => {
                        // c.fsdsp
                        let rs2 = (inst >> 2) & 0x1f;
                        // uimm[5:3|8:6] = isnt[12:10|9:7]
                        let imm8_6 = (((inst & 0x380) as i32) as i64) >> 7;
                        let imm5_3 = (inst >> 10) & 0b111;
                        let uimm = (imm8_6 << 6) as u64 | imm5_3 << 3;
                        let addr = self.xregs.read(2) + uimm;
                        self.write64(addr, self.fregs.read(rs2).to_bits() as u64)?;
                    }
                    0x6 => {
                        // c.swsp
                        let rs2 = (inst >> 2) & 0x1f;
                        // uimm[5:2|7:6] = inst[12:9|8:7]
                        let imm7_6 = (((inst & 0x180) as i32) as i64) >> 7;
                        let imm5_2 = (inst >> 9) & 0b1111;
                        let uimm = (imm7_6 << 6) as u64 | imm5_2 << 2;
                        let addr = self.xregs.read(2).wrapping_add(uimm);
                        self.write32(addr, self.xregs.read(rs2))?;
                    }
                    0x7 => {
                        // c.sdsp
                        let rs2 = (inst >> 2) & 0x1f;
                        // uimm[5:3|8:6] = isnt[12:10|9:7]
                        let imm8_6 = (((inst & 0x380) as i32) as i64) >> 7;
                        let imm5_3 = (inst >> 10) & 0b111;
                        let uimm = (imm8_6 << 6) as u64 | imm5_3 << 3;
                        let addr = self.xregs.read(2) + uimm;
                        self.write64(addr, self.xregs.read(rs2))?;
                    }
                    _ => {}
                }
            }
            _ => {
                return Err(Exception::IllegalInstruction);
            }
        }
        Ok(())
    }

    /// Execute a general-purpose instruction. Raises an exception if something is wrong,
    /// otherwise, returns nothing. It also increments the program counter by 4 bytes.
    pub fn tick_g(&mut self) -> Result<(), Exception> {
        // 1. Fetch.
        let inst = self.fetch32()?;

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
                // I-type
                // imm[11:0] = inst[31:20]
                let offset = ((inst as i32 as i64) >> 20) as u64;
                let addr = self.xregs.read(rs1).wrapping_add(offset);
                match funct3 {
                    0x0 => {
                        // lb
                        let val = self.read8(addr)?;
                        self.xregs.write(rd, val as i8 as i64 as u64);
                    }
                    0x1 => {
                        // lh
                        let val = self.read16(addr)?;
                        self.xregs.write(rd, val as i16 as i64 as u64);
                    }
                    0x2 => {
                        // lw
                        let val = self.read32(addr)?;
                        self.xregs.write(rd, val as i32 as i64 as u64);
                    }
                    0x3 => {
                        // ld
                        let val = self.read64(addr)?;
                        self.xregs.write(rd, val);
                    }
                    0x4 => {
                        // lbu
                        let val = self.read8(addr)?;
                        self.xregs.write(rd, val);
                    }
                    0x5 => {
                        // lhu
                        let val = self.read16(addr)?;
                        self.xregs.write(rd, val);
                    }
                    0x6 => {
                        // lwu
                        let val = self.read32(addr)?;
                        self.xregs.write(rd, val);
                    }
                    _ => {}
                }
            }
            0x07 => {
                // I-type (RV32F and RV64F)
                // imm[11:0] = inst[31:20]
                let offset = match (inst & 0x80000000) == 0 {
                    true => 0,
                    false => 0xffffffff_fffff800,
                } | ((inst >> 20) & 0x000007ff);
                let addr = (self.xregs.read(rs1).wrapping_add(offset)) & 0xffffffff;
                match funct3 {
                    0x2 => {
                        // flw
                        let val = f64::from_bits(self.read64(addr)? as u64);
                        self.fregs.write(rd, val);
                    }
                    0x3 => {
                        // fld
                        let val = f64::from_bits(self.read64(addr)?);
                        self.fregs.write(rd, val);
                    }
                    _ => {}
                }
            }
            0x0F => {
                // I-type (RV32I)
                // fence instructions are not supportted yet because this emulator executes a
                // inst sequentially on a single thread.
                // fence i is a part of the Zifencei extension.
                match funct3 {
                    0x0 => {} // fence
                    0x1 => {} // fence.i
                    _ => {}
                }
            }
            0x13 => {
                // I-type (RV32I and RV64I)
                // imm[11:0] = inst[31:20]
                let imm = ((inst & 0xfff00000) as i32 as i64 >> 20) as u64;
                // shamt size is 5 bits for RV32I and 6 bits for RV64I.
                // let shamt = (inst & 0x01F00000) >> 20; // This is for RV32I
                let shamt = ((inst & 0x03f00000) >> 20) as u32;
                let funct6 = funct7 >> 1;
                match funct3 {
                    0x0 => {
                        // addi
                        self.xregs.write(rd, self.xregs.read(rs1).wrapping_add(imm));
                    }
                    0x1 => self.xregs.write(rd, self.xregs.read(rs1) << shamt), // slli
                    0x2 => self.xregs.write(
                        // slti
                        rd,
                        if (self.xregs.read(rs1) as i64) < (imm as i64) {
                            1
                        } else {
                            0
                        },
                    ),
                    0x3 => {
                        // sltiu
                        self.xregs
                            .write(rd, if self.xregs.read(rs1) < imm { 1 } else { 0 });
                    }
                    0x4 => self.xregs.write(rd, self.xregs.read(rs1) ^ imm), // xori
                    0x5 => {
                        match funct6 {
                            // srli
                            0x00 => self
                                .xregs
                                .write(rd, self.xregs.read(rs1).wrapping_shr(shamt)),
                            // srai
                            0x10 => self.xregs.write(
                                rd,
                                (self.xregs.read(rs1) as i64).wrapping_shr(shamt) as u64,
                            ),
                            _ => {}
                        }
                    }
                    0x6 => self.xregs.write(rd, self.xregs.read(rs1) | imm), // ori
                    0x7 => self.xregs.write(rd, self.xregs.read(rs1) & imm), // andi
                    _ => {}
                }
            }
            0x17 => {
                // U-type (RV32I)
                // AUIPC forms a 32-bit offset from the 20-bit U-immediate, filling
                // in the lowest 12 bits with zeros.
                let imm = (inst & 0xfffff000) as i32 as i64 as u64;
                // auipc
                self.xregs
                    .write(rd, self.pc.wrapping_add(imm).wrapping_sub(4));
            }
            0x1B => {
                // I-type (RV64I only)
                let imm = ((inst as i32 as i64) >> 20) as u64;
                // "SLLIW, SRLIW, and SRAIW encodings with imm[5] ̸= 0 are reserved."
                let shamt = (imm & 0x1f) as u32;
                match funct3 {
                    0x0 => {
                        // addiw
                        self.xregs.write(
                            rd,
                            self.xregs.read(rs1).wrapping_add(imm) as i32 as i64 as u64,
                        );
                    }
                    0x1 => self.xregs.write(
                        // slliw
                        rd,
                        self.xregs.read(rs1).wrapping_shl(shamt) as i32 as i64 as u64,
                    ),
                    0x5 => {
                        match funct7 {
                            0x00 => {
                                // srliw
                                self.xregs.write(
                                    rd,
                                    (self.xregs.read(rs1) as u32).wrapping_shr(shamt) as i32 as i64
                                        as u64,
                                )
                            }
                            0x20 => {
                                // sraiw
                                self.xregs.write(
                                    rd,
                                    (self.xregs.read(rs1) as i32).wrapping_shr(shamt) as i64 as u64,
                                );
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            0x23 => {
                // S-type (RV32I)
                // imm[11:5|4:0] = inst[31:25|11:7]
                let imm = (((inst & 0xfe000000) as i32 as i64 >> 20) as u64) | ((inst >> 7) & 0x1f);
                let addr = self.xregs.read(rs1).wrapping_add(imm);
                match funct3 {
                    0x0 => self.write8(addr, self.xregs.read(rs2))?, // sb
                    0x1 => self.write16(addr, self.xregs.read(rs2))?, // sh
                    0x2 => self.write32(addr, self.xregs.read(rs2))?, // sw
                    0x3 => self.write64(addr, self.xregs.read(rs2))?, // sd
                    _ => {}
                }
            }
            0x27 => {
                // S-type (RV32F and RV64F)
                let offset = match (inst & 0x80000000) == 0 {
                    true => 0,
                    false => 0xffffffff_ffff800,
                } | ((inst & 0xfe000000) >> 20)
                    | ((inst & 0x00000f80) >> 7);
                let addr = self.xregs.read(rs1).wrapping_add(offset);
                match funct3 {
                    0x2 => self
                        .bus
                        .write32(addr, (self.fregs.read(rs2) as f32).to_bits() as u64)?, // fsw
                    0x3 => self.write64(addr, self.fregs.read(rs2).to_bits() as u64)?, // fsd
                    _ => {}
                }
            }
            0x2F => {
                // R-type (RV32A and RV64A)
                let funct5 = (funct7 & 0b1111100) >> 2;
                let _aq = (funct7 & 0b0000010) >> 1; // acquire access
                let _rl = funct7 & 0b0000001; // release access
                match (funct3, funct5) {
                    // TODO: if the address is not naturally aligned, a misaligned address
                    // exception or an access exception will be generated.
                    (0x2, 0x00) => {
                        // amoadd.w
                        let t = self.read32(self.xregs.read(rs1))?;
                        self.write32(self.xregs.read(rs1), t.wrapping_add(self.xregs.read(rs2)))?;
                        self.xregs.write(rd, t);
                    }
                    (0x3, 0x00) => {
                        // amoadd.d
                        let t = self.read64(self.xregs.read(rs1))?;
                        self.write64(self.xregs.read(rs1), t.wrapping_add(self.xregs.read(rs2)))?;
                        self.xregs.write(rd, t);
                    }
                    (0x2, 0x01) => {
                        // amoswap.w
                        let t = self.read32(self.xregs.read(rs1))?;
                        self.bus
                            .write32(self.xregs.read(rs1), self.xregs.read(rs2))?;
                        self.xregs.write(rd, t);
                    }
                    (0x3, 0x01) => {
                        // amoswap.d
                        let t = self.read64(self.xregs.read(rs1))?;
                        self.bus
                            .write64(self.xregs.read(rs1), self.xregs.read(rs2))?;
                        self.xregs.write(rd, t);
                    }
                    (0x2, 0x02) => {
                        // lr.w
                        let addr = self.read32(self.xregs.read(rs1))?;
                        self.xregs.write(rd, addr);
                    }
                    (0x3, 0x02) => {
                        // lr.d
                        let addr = self.read32(self.xregs.read(rs1))?;
                        self.xregs.write(rd, addr);
                    }
                    (0x2, 0x03) => {
                        // TODO: write a nonzero error code if the store fails.
                        // sc.w
                        let addr = self.read32(self.xregs.read(rs1) as u64)?;
                        let src = self.read32(self.xregs.read(rs2) as u64)?;
                        self.xregs.write(rd, 0);
                        self.bus.write32(addr as u64, src as u64)?;
                    }
                    (0x3, 0x03) => {
                        // TODO: write a nonzero error code if the store fails.
                        // sc.d
                        let addr = self.read32(self.xregs.read(rs1))?;
                        let src = self.read32(self.xregs.read(rs2))?;
                        self.xregs.write(rd, 0);
                        self.bus.write64(addr, src)?;
                    }
                    (0x2, 0x04) => {
                        // amoxor.w
                        let t = self.read32(self.xregs.read(rs1))? as u32;
                        self.write32(
                            self.xregs.read(rs1),
                            (t ^ (self.xregs.read(rs2) as u32)) as u64,
                        )?;
                        self.xregs.write(rd, t as u64);
                    }
                    (0x3, 0x04) => {
                        // amoxor.d
                        let t = self.read64(self.xregs.read(rs1))?;
                        self.write64(self.xregs.read(rs1), t ^ self.xregs.read(rs2))?;
                        self.xregs.write(rd, t);
                    }
                    (0x2, 0x08) => {
                        // amoor.w
                        let t = self.read32(self.xregs.read(rs1))? as i32;
                        self.write32(
                            self.xregs.read(rs1),
                            (t | (self.xregs.read(rs2) as i32)) as u32 as u64,
                        )?;
                        self.xregs.write(rd, t as u64);
                    }
                    (0x3, 0x08) => {
                        // amoor.d
                        let t = self.read64(self.xregs.read(rs1))?;
                        self.write64(self.xregs.read(rs1), t | self.xregs.read(rs2))?;
                        self.xregs.write(rd, t);
                    }
                    (0x2, 0x0c) => {
                        // amoand.w
                        let t = self.read32(self.xregs.read(rs1))? as i32;
                        self.write32(
                            self.xregs.read(rs1),
                            (t & (self.xregs.read(rs2) as i32)) as u32 as u64,
                        )?;
                        self.xregs.write(rd, t as u64);
                    }
                    (0x3, 0x0c) => {
                        // amoand.d
                        let t = self.read64(self.xregs.read(rs1))?;
                        self.write64(self.xregs.read(rs1), t & self.xregs.read(rs1))?;
                        self.xregs.write(rd, t);
                    }
                    (0x2, 0x10) => {
                        // amomin.w
                        let t = self.read32(self.xregs.read(rs1))? as i32;
                        self.write32(
                            self.xregs.read(rs1),
                            cmp::min(t, self.xregs.read(rs2) as i32) as u32 as u64,
                        )?;
                        self.xregs.write(rd, t as u64);
                    }
                    (0x3, 0x10) => {
                        // amomin.d
                        let t = self.read64(self.xregs.read(rs1))? as i64;
                        self.write64(
                            self.xregs.read(rs1),
                            cmp::min(t, self.xregs.read(rs2) as i64) as u64,
                        )?;
                        self.xregs.write(rd, t as u64);
                    }
                    (0x2, 0x14) => {
                        // amomax.w
                        let t = self.read32(self.xregs.read(rs1))? as i32;
                        self.write32(
                            self.xregs.read(rs1),
                            cmp::max(t, self.xregs.read(rs2) as i32) as u64,
                        )?;
                        self.xregs.write(rd, t as u64);
                    }
                    (0x3, 0x14) => {
                        // amomax.d
                        let t = self.read64(self.xregs.read(rs1))?;
                        self.write64(self.xregs.read(rs1), cmp::max(t, self.xregs.read(rs2)))?;
                        self.xregs.write(rd, t);
                    }
                    (0x2, 0x18) => {
                        // amominu.w
                        let t = self.read32(self.xregs.read(rs1))?;
                        self.write32(self.xregs.read(rs1), cmp::min(t, self.xregs.read(rs2)))?;
                        self.xregs.write(rd, t);
                    }
                    (0x3, 0x18) => {
                        // amominu.d
                        let t = self.read64(self.xregs.read(rs1))?;
                        self.write64(self.xregs.read(rs1), cmp::min(t, self.xregs.read(rs2)))?;
                        self.xregs.write(rd, t);
                    }
                    (0x2, 0x1c) => {
                        // amomaxu.w
                        let t = self.read32(self.xregs.read(rs1))?;
                        self.write32(self.xregs.read(rs1), cmp::max(t, self.xregs.read(rs2)))?;
                        self.xregs.write(rd, t);
                    }
                    (0x3, 0x1c) => {
                        // amomaxu.d
                        let t = self.read64(self.xregs.read(rs1))?;
                        self.write64(self.xregs.read(rs1), cmp::max(t, self.xregs.read(rs2)))?;
                        self.xregs.write(rd, t);
                    }
                    _ => {}
                }
            }
            0x33 => {
                // R-type (RV64I and RV64M)
                // "SLL, SRL, and SRA perform logical left, logical right, and arithmetic right
                // shifts on the value in register rs1 by the shift amount held in register rs2.
                // In RV64I, only the low 6 bits of rs2 are considered for the shift amount."
                let shamt = ((self.xregs.read(rs2) & 0x3f) as u64) as u32;
                match (funct3, funct7) {
                    (0x0, 0x00) => self
                        .xregs
                        .write(rd, self.xregs.read(rs1).wrapping_add(self.xregs.read(rs2))), // add
                    (0x0, 0x01) => self
                        .xregs
                        .write(rd, self.xregs.read(rs1).wrapping_mul(self.xregs.read(rs2))), // mul
                    (0x0, 0x20) => self
                        .xregs
                        .write(rd, self.xregs.read(rs1).wrapping_sub(self.xregs.read(rs2))), // sub
                    (0x1, 0x00) => self
                        .xregs
                        .write(rd, self.xregs.read(rs1).wrapping_shl(shamt)), // sll
                    (0x1, 0x01) => {
                        // mulh
                        self.xregs.write(
                            rd,
                            ((self.xregs.read(rs1) as i64 as i128)
                                .wrapping_mul(self.xregs.read(rs2) as i64 as i128)
                                >> 64) as u64,
                        );
                    }
                    (0x2, 0x00) => self.xregs.write(
                        // slt
                        rd,
                        if (self.xregs.read(rs1) as i64) < (self.xregs.read(rs2) as i64) {
                            1
                        } else {
                            0
                        },
                    ),
                    (0x2, 0x01) => {
                        // mulhsu
                        let x = self.xregs.read(rs1) as i64;
                        let y = self.xregs.read(rs2);
                        let z;
                        if x < 0 {
                            z = (!(-x as u64 as u128).wrapping_mul(y as u128)).wrapping_add(1);
                        } else {
                            z = (x as u64 as u128).wrapping_mul(y as u128);
                        }
                        self.xregs.write(rd, (z >> 64) as u64);
                    }
                    (0x3, 0x00) => {
                        // sltu
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
                        self.xregs.write(
                            rd,
                            ((self.xregs.read(rs1) as u128)
                                .wrapping_mul(self.xregs.read(rs2) as u128)
                                >> 64) as u64,
                        );
                    }
                    (0x4, 0x00) => self
                        .xregs
                        .write(rd, self.xregs.read(rs1) ^ self.xregs.read(rs2)), // xor
                    (0x4, 0x01) => {
                        // div
                        self.xregs.write(
                            rd,
                            match self.xregs.read(rs2) {
                                0 => {
                                    // Set DZ (Divide by Zero) flag to 1.
                                    self.state.write_bit(FCSR, 3, true);
                                    0xffffffff_ffffffff
                                }
                                _ => {
                                    let dividend = self.xregs.read(rs1) as i64;
                                    let divisor = self.xregs.read(rs2) as i64;
                                    dividend.wrapping_div(divisor) as u64
                                }
                            },
                        );
                    }
                    (0x5, 0x00) => self
                        .xregs
                        .write(rd, self.xregs.read(rs1).wrapping_shr(shamt)), // srl
                    (0x5, 0x01) => {
                        // divu
                        self.xregs.write(
                            rd,
                            match self.xregs.read(rs2) {
                                0 => {
                                    // Set DZ (Divide by Zero) flag to 1.
                                    self.state.write_bit(FCSR, 3, true);
                                    0xffffffff_ffffffff
                                }
                                _ => {
                                    let dividend = self.xregs.read(rs1);
                                    let divisor = self.xregs.read(rs2);
                                    dividend.wrapping_div(divisor)
                                }
                            },
                        );
                    }
                    (0x5, 0x20) => self
                        .xregs
                        .write(rd, (self.xregs.read(rs1) as i64).wrapping_shr(shamt) as u64), // sra
                    (0x6, 0x00) => self
                        .xregs
                        .write(rd, self.xregs.read(rs1) | self.xregs.read(rs2)), // or
                    (0x6, 0x01) => self.xregs.write(
                        // rem
                        rd,
                        match self.xregs.read(rs2) {
                            0 => self.xregs.read(rs1),
                            _ => {
                                let dividend = self.xregs.read(rs1) as i64;
                                let divisor = self.xregs.read(rs2) as i64;
                                dividend.wrapping_rem(divisor) as u64
                            }
                        },
                    ),
                    (0x7, 0x00) => self
                        .xregs
                        .write(rd, self.xregs.read(rs1) & self.xregs.read(rs2)), // and
                    (0x7, 0x01) => {
                        // remu
                        self.xregs.write(
                            rd,
                            match self.xregs.read(rs2) {
                                0 => self.xregs.read(rs1),
                                _ => {
                                    let dividend = self.xregs.read(rs1);
                                    let divisor = self.xregs.read(rs2);
                                    dividend.wrapping_rem(divisor)
                                }
                            },
                        );
                    }
                    _ => {}
                };
            }
            0x37 => {
                // U-type (RV32I)
                // LUI places the U-immediate value in the top 20 bits of the destination
                // register rd, filling in the lowest 12 bits with zeros.
                self.xregs
                    .write(rd, (inst & 0xfffff000) as i32 as i64 as u64); // lui
            }
            0x3B => {
                // R-type (RV64I and RV64M)
                // The shift amount is given by rs2[4:0].
                let shamt = (self.xregs.read(rs2) & 0x1f) as u32;
                match (funct3, funct7) {
                    (0x0, 0x00) => {
                        // addw
                        self.xregs.write(
                            rd,
                            self.xregs.read(rs1).wrapping_add(self.xregs.read(rs2)) as i32 as i64
                                as u64,
                        );
                    }
                    (0x0, 0x01) => {
                        // mulw
                        let n1 = self.xregs.read(rs1) as i32;
                        let n2 = self.xregs.read(rs2) as i32;
                        let result = n1.wrapping_mul(n2);
                        self.xregs.write(rd, result as u64);
                    }
                    (0x0, 0x20) => {
                        // subw
                        self.xregs.write(
                            rd,
                            ((self.xregs.read(rs1).wrapping_sub(self.xregs.read(rs2))) as i32)
                                as u64,
                        );
                    }
                    (0x1, 0x00) => {
                        // sllw
                        self.xregs.write(
                            rd,
                            (self.xregs.read(rs1) as u32).wrapping_shl(shamt) as i32 as i64 as u64,
                        );
                    }
                    (0x4, 0x01) => {
                        // divw
                        self.xregs.write(
                            rd,
                            match self.xregs.read(rs2) {
                                0 => {
                                    // Set DZ (Divide by Zero) flag to 1.
                                    self.state.write_bit(FCSR, 3, true);
                                    0xffffffff_ffffffff
                                }
                                _ => {
                                    let dividend = self.xregs.read(rs1) as i32;
                                    let divisor = self.xregs.read(rs2) as i32;
                                    dividend.wrapping_div(divisor) as u64
                                }
                            },
                        );
                    }
                    (0x5, 0x00) => {
                        // srlw
                        self.xregs.write(
                            rd,
                            (self.xregs.read(rs1) as u32).wrapping_shr(shamt) as i32 as i64 as u64,
                        );
                    }
                    (0x5, 0x01) => {
                        // divuw
                        self.xregs.write(
                            rd,
                            match self.xregs.read(rs2) {
                                0 => {
                                    // Set DZ (Divide by Zero) flag to 1.
                                    self.state.write_bit(FCSR, 3, true);
                                    0xffffffff_ffffffff
                                }
                                _ => {
                                    let dividend = self.xregs.read(rs1) as u32;
                                    let divisor = self.xregs.read(rs2) as u32;
                                    (dividend.wrapping_div(divisor) as i32) as u64
                                }
                            },
                        );
                    }
                    (0x5, 0x20) => {
                        // sraw
                        self.xregs.write(
                            rd,
                            (self.xregs.read(rs1) as i32).wrapping_shr(shamt) as i64 as u64,
                        );
                    }
                    (0x6, 0x01) => {
                        // remw
                        self.xregs.write(
                            rd,
                            match self.xregs.read(rs2) {
                                0 => self.xregs.read(rs1),
                                _ => {
                                    let dividend = self.xregs.read(rs1) as i32;
                                    let divisor = self.xregs.read(rs2) as i32;
                                    dividend.wrapping_rem(divisor) as u64
                                }
                            },
                        );
                    }
                    (0x7, 0x01) => {
                        // remuw
                        self.xregs.write(
                            rd,
                            match self.xregs.read(rs2) {
                                0 => self.xregs.read(rs1),
                                _ => {
                                    let dividend = self.xregs.read(rs1) as u32;
                                    let divisor = self.xregs.read(rs2) as u32;
                                    dividend.wrapping_rem(divisor) as i32 as u64
                                }
                            },
                        );
                    }
                    _ => {}
                }
            }
            0x43 => {
                // R4-type (RV32F and RV64F)
                // TODO: support the rounding mode encoding (rm).
                let rs3 = ((inst & 0xf8000000) >> 27) as u64;
                let funct2 = (inst & 0x03000000) >> 25;
                match funct2 {
                    0x0 => {
                        // fmadd.s
                        self.fregs.write(
                            rd,
                            (self.fregs.read(rs1) as f32)
                                .mul_add(self.fregs.read(rs2) as f32, self.fregs.read(rs3) as f32)
                                as f64,
                        );
                    }
                    0x1 => self.fregs.write(
                        rd,
                        self.fregs
                            .read(rs1)
                            .mul_add(self.fregs.read(rs2), self.fregs.read(rs3)),
                    ), // fmadd.d
                    _ => {}
                }
            }
            0x47 => {
                // R4-type (RV32F and RV64F)
                // TODO: support the rounding mode encoding (rm).
                let rs3 = ((inst & 0xf8000000) >> 27) as u64;
                let funct2 = (inst & 0x03000000) >> 25;
                match funct2 {
                    0x0 => {
                        // fmsub.s
                        self.fregs.write(
                            rd,
                            (self.fregs.read(rs1) as f32)
                                .mul_add(self.fregs.read(rs2) as f32, -self.fregs.read(rs3) as f32)
                                as f64,
                        );
                    }
                    0x1 => self.fregs.write(
                        rd,
                        self.fregs
                            .read(rs1)
                            .mul_add(self.fregs.read(rs2), -self.fregs.read(rs3)),
                    ), // fmsub.d
                    _ => {}
                }
            }
            0x4B => {
                // R4-type (RV32F and RV64F)
                // TODO: support the rounding mode encoding (rm).
                let rs3 = ((inst & 0xf8000000) >> 27) as u64;
                let funct2 = (inst & 0x03000000) >> 25;
                match funct2 {
                    0x0 => {
                        // fnmadd.s
                        self.fregs.write(
                            rd,
                            (-self.fregs.read(rs1) as f32)
                                .mul_add(self.fregs.read(rs2) as f32, self.fregs.read(rs3) as f32)
                                as f64,
                        );
                    }
                    0x1 => self.fregs.write(
                        rd,
                        (-self.fregs.read(rs1)).mul_add(self.fregs.read(rs2), self.fregs.read(rs3)),
                    ), // fnmadd.d
                    _ => {}
                }
            }
            0x4F => {
                // R4-type (RV32F and RV64F)
                // TODO: support the rounding mode encoding (rm).
                let rs3 = ((inst & 0xf8000000) >> 27) as u64;
                let funct2 = (inst & 0x03000000) >> 25;
                match funct2 {
                    0x0 => {
                        // fnmsub.s
                        self.fregs.write(
                            rd,
                            (-self.fregs.read(rs1) as f32)
                                .mul_add(self.fregs.read(rs2) as f32, -self.fregs.read(rs3) as f32)
                                as f64,
                        );
                    }
                    0x1 => self.fregs.write(
                        rd,
                        (-self.fregs.read(rs1))
                            .mul_add(self.fregs.read(rs2), -self.fregs.read(rs3)),
                    ), // fnmsub.d
                    _ => {}
                }
            }
            0x53 => {
                // R-type (RV32F and RV64F)
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
                        self.fregs.write(
                            rd,
                            (self.fregs.read(rs1) as f32 + self.fregs.read(rs2) as f32) as f64,
                        )
                    }
                    0x01 => self
                        .fregs
                        .write(rd, self.fregs.read(rs1) + self.fregs.read(rs2)), // fadd.d
                    0x04 => {
                        // fsub.s
                        self.fregs.write(
                            rd,
                            (self.fregs.read(rs1) as f32 - self.fregs.read(rs2) as f32) as f64,
                        )
                    }
                    0x05 => self
                        .fregs
                        .write(rd, self.fregs.read(rs1) - self.fregs.read(rs2)), // fsub.d
                    0x08 => {
                        // fmul.s
                        self.fregs.write(
                            rd,
                            (self.fregs.read(rs1) as f32 * self.fregs.read(rs2) as f32) as f64,
                        )
                    }
                    0x09 => self
                        .fregs
                        .write(rd, self.fregs.read(rs1) * self.fregs.read(rs2)), // fmul.d
                    0x0c => {
                        // fdiv.s
                        self.fregs.write(
                            rd,
                            (self.fregs.read(rs1) as f32 / self.fregs.read(rs2) as f32) as f64,
                        )
                    }
                    0x0d => self
                        .fregs
                        .write(rd, self.fregs.read(rs1) / self.fregs.read(rs2)), // fdiv.d
                    0x10 => {
                        match funct3 {
                            0x0 => self
                                .fregs
                                .write(rd, self.fregs.read(rs1).copysign(self.fregs.read(rs2))), // fsgnj.s
                            0x1 => self
                                .fregs
                                .write(rd, self.fregs.read(rs1).copysign(-self.fregs.read(rs2))), // fsgnjn.s
                            0x2 => {
                                let sign1 = (self.fregs.read(rs1) as f32).to_bits() & 0x80000000;
                                let sign2 = (self.fregs.read(rs2) as f32).to_bits() & 0x80000000;
                                let other = (self.fregs.read(rs1) as f32).to_bits() & 0x7fffffff;
                                // fsgnjx.s
                                self.fregs
                                    .write(rd, f32::from_bits((sign1 ^ sign2) | other) as f64);
                            }
                            _ => {}
                        }
                    }
                    0x11 => {
                        match funct3 {
                            0x0 => self
                                .fregs
                                .write(rd, self.fregs.read(rs1).copysign(self.fregs.read(rs2))), // fsgnj.d
                            0x1 => self
                                .fregs
                                .write(rd, self.fregs.read(rs1).copysign(-self.fregs.read(rs2))), // fsgnjn.d
                            0x2 => {
                                let sign1 = self.fregs.read(rs1).to_bits() & 0x80000000_00000000;
                                let sign2 = self.fregs.read(rs2).to_bits() & 0x80000000_00000000;
                                let other = self.fregs.read(rs1).to_bits() & 0x7fffffff_ffffffff;
                                // fsgnjx.d
                                self.fregs
                                    .write(rd, f64::from_bits((sign1 ^ sign2) | other));
                            }
                            _ => {}
                        }
                    }
                    0x14 => {
                        match funct3 {
                            0x0 => self
                                .fregs
                                .write(rd, self.fregs.read(rs1).min(self.fregs.read(rs2))), // fmin.s
                            0x1 => self
                                .fregs
                                .write(rd, self.fregs.read(rs1).max(self.fregs.read(rs2))), // fmax.s
                            _ => {}
                        }
                    }
                    0x15 => {
                        match funct3 {
                            0x0 => self
                                .fregs
                                .write(rd, self.fregs.read(rs1).min(self.fregs.read(rs2))), // fmin.d
                            0x1 => self
                                .fregs
                                .write(rd, self.fregs.read(rs1).max(self.fregs.read(rs2))), // fmax.d
                            _ => {}
                        }
                    }
                    0x20 => self.fregs.write(rd, self.fregs.read(rs1)), // fcvt.s.d
                    0x21 => self.fregs.write(rd, (self.fregs.read(rs1) as f32) as f64), // fcvt.d.s
                    0x2c => self
                        .fregs
                        .write(rd, (self.fregs.read(rs1) as f32).sqrt() as f64), // fsqrt.s
                    0x2d => self.fregs.write(rd, self.fregs.read(rs1).sqrt()), // fsqrt.d
                    0x50 => {
                        match funct3 {
                            0x0 => self.xregs.write(
                                rd,
                                if self.fregs.read(rs1) <= self.fregs.read(rs2) {
                                    1
                                } else {
                                    0
                                },
                            ), // fle.s
                            0x1 => self.xregs.write(
                                rd,
                                if self.fregs.read(rs1) < self.fregs.read(rs2) {
                                    1
                                } else {
                                    0
                                },
                            ), // flt.s
                            0x2 => self.xregs.write(
                                rd,
                                if self.fregs.read(rs1) == self.fregs.read(rs2) {
                                    1
                                } else {
                                    0
                                },
                            ), // feq.s
                            _ => {}
                        }
                    }
                    0x51 => {
                        match funct3 {
                            0x0 => self.xregs.write(
                                rd,
                                if self.fregs.read(rs1) <= self.fregs.read(rs2) {
                                    1
                                } else {
                                    0
                                },
                            ), // fle.d
                            0x1 => self.xregs.write(
                                rd,
                                if self.fregs.read(rs1) < self.fregs.read(rs2) {
                                    1
                                } else {
                                    0
                                },
                            ), // flt.d
                            0x2 => self.xregs.write(
                                rd,
                                if self.fregs.read(rs1) == self.fregs.read(rs2) {
                                    1
                                } else {
                                    0
                                },
                            ), // feq.d
                            _ => {}
                        }
                    }
                    0x60 => {
                        match rs2 {
                            0x0 => self
                                .xregs
                                .write(rd, ((self.fregs.read(rs1) as f32).round() as i32) as u64), // fcvt.w.s
                            0x1 => self.xregs.write(
                                rd,
                                (((self.fregs.read(rs1) as f32).round() as u32) as i32) as u64,
                            ), // fcvt.wu.s
                            0x2 => self
                                .xregs
                                .write(rd, (self.fregs.read(rs1) as f32).round() as u64), // fcvt.l.s
                            0x3 => self
                                .xregs
                                .write(rd, (self.fregs.read(rs1) as f32).round() as u64), // fcvt.lu.s
                            _ => {}
                        }
                    }
                    0x61 => {
                        match rs2 {
                            0x0 => self
                                .xregs
                                .write(rd, (self.fregs.read(rs1).round() as i32) as u64), // fcvt.w.d
                            0x1 => self
                                .xregs
                                .write(rd, ((self.fregs.read(rs1).round() as u32) as i32) as u64), // fcvt.wu.d
                            0x2 => self.xregs.write(rd, self.fregs.read(rs1).round() as u64), // fcvt.l.d
                            0x3 => self.xregs.write(rd, self.fregs.read(rs1).round() as u64), // fcvt.lu.d
                            _ => {}
                        }
                    }
                    0x68 => {
                        match rs2 {
                            0x0 => self
                                .fregs
                                .write(rd, ((self.xregs.read(rs1) as i32) as f32) as f64), // fcvt.s.w
                            0x1 => self
                                .fregs
                                .write(rd, ((self.xregs.read(rs1) as u32) as f32) as f64), // fcvt.s.wu
                            0x2 => self.fregs.write(rd, (self.xregs.read(rs1) as f32) as f64), // fcvt.s.l
                            0x3 => self
                                .fregs
                                .write(rd, ((self.xregs.read(rs1) as u64) as f32) as f64), // fcvt.s.lu
                            _ => {}
                        }
                    }
                    0x69 => {
                        match rs2 {
                            0x0 => self.fregs.write(rd, (self.xregs.read(rs1) as i32) as f64), // fcvt.d.w
                            0x1 => self.fregs.write(rd, (self.xregs.read(rs1) as u32) as f64), // fcvt.d.wu
                            0x2 => self.fregs.write(rd, self.xregs.read(rs1) as f64), // fcvt.d.l
                            0x3 => self.fregs.write(rd, self.xregs.read(rs1) as f64), // fcvt.d.lu
                            _ => {}
                        }
                    }
                    0x70 => {
                        match funct3 {
                            0x0 => self.xregs.write(rd, (self.fregs.read(rs1) as i32) as u64), // fmv.x.w
                            0x1 => {
                                // fclass.s
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
                            _ => {}
                        }
                    }
                    0x71 => {
                        match funct3 {
                            0x0 => self.xregs.write(rd, self.fregs.read(rs1) as u64), // fmv.x.d
                            0x1 => {
                                // fclass.d
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
                            _ => {}
                        }
                    }
                    0x78 => self
                        .fregs
                        .write(rd, ((self.xregs.read(rs1) as i32) as f32) as f64), // fmv.w.x
                    0x79 => self.fregs.write(rd, self.xregs.read(rs1) as f64), // fmv.d.x
                    _ => {}
                }
            }
            0x63 => {
                // B-type (RV32I)
                // imm[12|10:5|4:1|11] = inst[31|30:25|11:8|7]
                let imm = (((inst & 0x80000000) as i32 as i64 >> 19) as u64)
                    | ((inst & 0x80) << 4) // imm[11]
                    | ((inst >> 20) & 0x7e0) // imm[10:5]
                    | ((inst >> 7) & 0x1e); // imm[4:1]

                match funct3 {
                    0x0 => {
                        // beq
                        if self.xregs.read(rs1) == self.xregs.read(rs2) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x1 => {
                        // bne
                        if self.xregs.read(rs1) != self.xregs.read(rs2) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x4 => {
                        // blt
                        if (self.xregs.read(rs1) as i64) < (self.xregs.read(rs2) as i64) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x5 => {
                        // bge
                        if (self.xregs.read(rs1) as i64) >= (self.xregs.read(rs2) as i64) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x6 => {
                        // bltu
                        if self.xregs.read(rs1) < self.xregs.read(rs2) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    0x7 => {
                        // bgeu
                        if self.xregs.read(rs1) >= self.xregs.read(rs2) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }
                    _ => {}
                }
            }
            0x67 => {
                // I-type
                // jalr
                let t = self.pc;

                let offset = (((inst & 0xfff00000) as i32) as i64) >> 20;
                let target = ((self.xregs.read(rs1) as i64).wrapping_add(offset)) & !1;

                self.pc = target as u64;
                self.xregs.write(rd, t);
            }
            0x6F => {
                // J-type
                // jal
                self.xregs.write(rd, self.pc);

                // imm[20|10:1|11|19:12] = inst[31|30:21|20|19:12]
                let offset = (((inst & 0x80000000) as i32 as i64 >> 11) as u64) // imm[20]
                    | (inst & 0xff000) // imm[19:12]
                    | ((inst >> 9) & 0x800) // imm[11]
                    | ((inst >> 20) & 0x7fe); // imm[10:1]

                self.pc = self.pc.wrapping_add(offset).wrapping_sub(4);
            }
            0x73 => {
                // I-type (RV32I, RVZicsr, supervisor ISA)
                let csr_addr = ((inst & 0xfff00000) >> 20) as u16;
                match funct3 {
                    0x0 => {
                        match (rs2, funct7) {
                            (0x0, 0x0) => {
                                // ecall
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
                                    _ => {}
                                }
                            }
                            (0x1, 0x0) => {
                                // ebreak
                                // Makes a request of the debugger bu raising a Breakpoint
                                // exception.
                                return Err(Exception::Breakpoint);
                            }
                            (0x2, 0x0) => {
                                // uret
                                dbg!("uret: not implemented yet. pc {}", self.pc);
                            }
                            (0x2, 0x8) => {
                                // sret
                                // "The RISC-V Reader" book says:
                                // "Returns from a supervisor-mode exception handler. Sets the pc to
                                // CSRs[scpc], the privilege mode to CSRs[sstatus].SPP,
                                // CSRs[sstatus].SIE to CSRs[sstatus].SPIE, CSRs[sstatus].SPIE to
                                // 1, and CSRs[sstatus].SPP to 0.", but
                                // the implementation in QEMU and Spike use `mstatus` instead of
                                // `sstatus`.
                                self.mode.require(Mode::Supervisor)?;

                                // Set the program coutner to the supervisor exception program
                                // counter (SEPC).
                                self.pc = self.state.read(SEPC) as u64;

                                // TODO: Check TSR field

                                // Set the current privileged mode depending on a privious privilege mode for supervisor mode (SPP, 8).
                                self.mode = match self.state.read_bit(SSTATUS, 8) {
                                    false => Mode::User,
                                    true => Mode::Supervisor,
                                };
                                // Read a privious interrupt-enable bit for supervisor mode (SPIE, 5), and set a global interrupt-enable bit for supervisor mode (SIE, 1) to it.
                                self.state
                                    .write_bit(SSTATUS, 1, self.state.read_bit(SSTATUS, 5));

                                // Set a privious interrupt-enable bit for supervisor mode (SPIE,
                                // 5) to 1.
                                self.state.write_bit(SSTATUS, 5, true);
                                // Set a privious privilege mode for supervisor mode (SPP, 8) to 0.
                                self.state.write_bit(SSTATUS, 8, false);
                            }
                            (0x2, 0x18) => {
                                // mret
                                // "The RISC-V Reader" book says:
                                // "Returns from a machine-mode exception handler. Sets the pc to CSRs[mepc], the privilege mode to
                                // CSRs[mstatus].MPP, CSRs[mstatus].MIE to CSRs[mstatus].MPIE, and
                                // CSRs[mstatus].MPIE to 1; and, if user mode is supported, sets
                                // CSRs[mstatus].MPP to 0".
                                self.mode.require(Mode::Machine)?;

                                // Set the program coutner to the machine exception program
                                // counter (MEPC).
                                self.pc = self.state.read(MEPC) as u64;

                                // Set the current privileged mode depending on a privious privilege mode for machine  mode (MPP, 11..13).
                                self.mode = match self.state.read_bits(MSTATUS, 11..13) {
                                    0b00 => Mode::User,
                                    0b01 => Mode::Supervisor,
                                    0b11 => Mode::Machine,
                                    _ => Mode::Debug,
                                };

                                // Read a privious interrupt-enable bit for machine mode (MPIE, 7), and set a global interrupt-enable bit for machine mode (MIE, 3) to it.
                                self.state
                                    .write_bit(MSTATUS, 3, self.state.read_bit(MSTATUS, 7));

                                // Set a privious interrupt-enable bit for machine mode (MPIE, 7)
                                // to 1.
                                self.state.write_bit(MSTATUS, 7, true);

                                // Set a privious privilege mode for machine mode (MPP, 11..13) to
                                // 0.
                                self.state.write_bits(MSTATUS, 11..13, 0b00);
                            }
                            (0x5, 0x8) => {} // wfi
                            (_, 0x9) => {}   // sfence.vma
                            (_, 0x11) => {}  // hfence.bvma
                            (_, 0x51) => {}  // hfence.gvma
                            _ => {}
                        }
                    }
                    0x1 => {
                        // csrrw
                        let t = self.state.read(csr_addr);
                        self.state.write(csr_addr, self.xregs.read(rs1));
                        self.xregs.write(rd, t);

                        if csr_addr == SATP {
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
                    }
                    0x2 => {
                        // csrrs
                        self.xregs.write(rd, self.state.read(csr_addr));
                        self.state
                            .write(csr_addr, self.xregs.read(rd) | self.xregs.read(rs1));
                    }
                    0x3 => {
                        // csrrc
                        self.xregs.write(rd, self.state.read(csr_addr));
                        self.state
                            .write(csr_addr, self.xregs.read(rd) & (!self.xregs.read(rs1)));
                    }
                    0x5 => {
                        // csrrwi
                        self.xregs.write(rd, self.state.read(csr_addr));
                        self.state.write(csr_addr, rs1);

                        if csr_addr == SATP {
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
                    }
                    0x6 => {
                        // csrrsi
                        self.xregs.write(rd, self.state.read(csr_addr));
                        self.state.write(csr_addr, self.xregs.read(rd) | rs1);
                    }
                    0x7 => {
                        // csrrci
                        self.xregs.write(rd, self.state.read(csr_addr));
                        self.state.write(csr_addr, self.xregs.read(rd) & !rs1);
                    }
                    _ => {}
                }
            }
            _ => {
                return Err(Exception::IllegalInstruction);
            }
        }
        Ok(())
    }
}
