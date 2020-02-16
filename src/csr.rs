use std::collections::HashMap;

// Machine trap setup.
const MSTATUS: u32 = 0x300; // Machine status register.
const MISA: u32 = 0x301; // ISA and extensions.
const MEDELEG: u32 = 0x302; // Machine exception delefation register.
const MIDELEG: u32 = 0x303; // Machine interrupt delefation register.
const MIE: u32 = 0x304; // Machine interrupt-enable register.
const MTVEC: u32 = 0x305; // Machine trap-handler base address.

// Machine trap handling.
const MEPC: u32 = 0x342; // Machine exception program counter.
const MCAUSE: u32 = 0x342; // Machine trap cause.
const MIP: u32 = 0x344; // Machine interrupt pending.

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

    pub fn read(&self, csr_address: u32) -> i64 {
        *self.regs.get(&csr_address).expect("failed to read a csr")
    }

    pub fn write(&mut self, csr_address: u32, value: i64) {
        let original = self
            .regs
            .get_mut(&csr_address)
            .expect("failed to get a csr");
        *original = value;
    }
}
