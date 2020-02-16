pub mod fcsr;

use std::collections::HashMap;

use crate::exception::Exception;

// ***** User-level CSR addresses *****
// User trap handling.
pub const UEPC: u32 = 0x041; // User exception program counter.
pub const UCAUSE: u32 = 0x042; // User trap cause.

// User floating-point CSRs.
pub const FFLAGS: u32 = 0x001; // Flating-point accrued exceptions.
pub const FRB: u32 = 0x002; // Floating-point dynamic rounding mode.
pub const FCSR: u32 = 0x003; // Floating-point control and status register (frm + fflags).

// ***** Supervisor-level CSR addresses *****
// Supervisor trap handling.
pub const SEPC: u32 = 0x141; // Supervisor exception program counter.
pub const SCAUSE: u32 = 0x142; // Supervisor trap cause.

// ***** Machine-level CSR addresses *****
// Machine trap setup.
pub const MSTATUS: u32 = 0x300; // Machine status register.
pub const MISA: u32 = 0x301; // ISA and extensions.
pub const MEDELEG: u32 = 0x302; // Machine exception delefation register.
pub const MIDELEG: u32 = 0x303; // Machine interrupt delefation register.
pub const MIE: u32 = 0x304; // Machine interrupt-enable register.
pub const MTVEC: u32 = 0x305; // Machine trap-handler base address.

// Machine trap handling.
pub const MEPC: u32 = 0x342; // Machine exception program counter.
pub const MCAUSE: u32 = 0x342; // Machine trap cause.
pub const MIP: u32 = 0x344; // Machine interrupt pending.

// Machine information registers.
const MHARTID: u32 = 0xf14; // Hardware thread ID.

pub struct Csr {
    regs: HashMap<u32, i64>,
}

impl Csr {
    pub fn new() -> Self {
        let mut regs = HashMap::new();

        // csr[11:10]: Whether the register is read/write (00, 01, or 10) or read-only (11).
        // csr[9:8]: The lowest privilege level that can access the CSR. User (00), supervisor
        // (01), hypervisor (10), and machine (11).
        regs.insert(0b01 << 00 | 0b00 << 8 | UEPC, 0);
        regs.insert(0b01 << 00 | 0b00 << 8 | UCAUSE, 0);
        regs.insert(0b00 << 00 | 0b00 << 8 | FFLAGS, 0);
        regs.insert(0b00 << 00 | 0b00 << 8 | FRB, 0);
        regs.insert(0b00 << 00 | 0b00 << 8 | FCSR, 0);

        regs.insert(0b00 << 00 | 0b01 << 8 | SEPC, 0);
        regs.insert(0b00 << 00 | 0b01 << 8 | SCAUSE, 0);

        regs.insert(0b00 << 00 | 0b11 << 8 | MSTATUS, 0);
        regs.insert(0b00 << 00 | 0b11 << 8 | MISA, 0);
        regs.insert(0b00 << 00 | 0b11 << 8 | MEDELEG, 0);
        regs.insert(0b00 << 00 | 0b11 << 8 | MIDELEG, 0);
        regs.insert(0b00 << 00 | 0b11 << 8 | MIE, 0);
        regs.insert(0b00 << 00 | 0b11 << 8 | MTVEC, 0);
        regs.insert(0b00 << 00 | 0b11 << 8 | MEPC, 0);
        regs.insert(0b00 << 00 | 0b11 << 8 | MCAUSE, 0);
        regs.insert(0b00 << 00 | 0b11 << 8 | MIP, 0);
        regs.insert(0b11 << 00 | 0b11 << 8 | MHARTID, 0); // read-only

        Self { regs }
    }

    pub fn read(&self, csr_address: u32) -> Result<i64, Exception> {
        if let Some(csr_val) = self.regs.get(&csr_address) {
            Ok(*csr_val)
        } else {
            Err(Exception::IllegalInstruction(String::from(
                "failed to read a csr.",
            )))
        }
    }

    pub fn write(&mut self, csr_address: u32, value: i64) -> Result<(), Exception> {
        if let Some(csr_val) = self.regs.get_mut(&csr_address) {
            Ok(*csr_val = value)
        } else {
            Err(Exception::IllegalInstruction(String::from(
                "failed to write a csr.",
            )))
        }
    }

    pub fn clear(&mut self) {
        for csr_val in self.regs.values_mut() {
            *csr_val = 0;
        }
    }
}
