use std::collections::HashMap;

// Machine trap setup.
const MSTATUS: u32 = 0x300; // Machine status register.
const MISA: u32 = 0x301; // Machine interrupt-enable register.
const MIE: u32 = 0x304; // Machine interrupt-enable register.
// Machine trap handling.
const MIP: u32 = 0x344; // Machine interrupt pending.
// Machine information registers.
const MHARTID: u32 = 0xf14; // Hardware thread ID.

pub struct Csr {
    regs: HashMap<u32, i64>,
}

impl Csr {
    pub fn new() -> Self {
        let mut regs = HashMap::new();
        Csr::init_csrs(&mut regs, MSTATUS);
        Csr::init_csrs(&mut regs, MISA);
        Csr::init_csrs(&mut regs, MIE);
        Csr::init_csrs(&mut regs, MIP);
        Csr::init_csrs(&mut regs, MHARTID);

        Self { regs }
    }

    fn init_csrs(regs: &mut HashMap<u32, i64>, csr_number: u32) {
        // csr[11:10]: Whether the register is read/write (00, 01, or 10) or read-only (11).
        // csr[9:8]: The lowest privilege level that can access the CSR.

        // User CSRs.
        regs.insert(0b00 << 10 | 0b00 << 8 | csr_number, 0);
        regs.insert(0b01 << 10 | 0b00 << 8 | csr_number, 0);
        regs.insert(0b10 << 10 | 0b00 << 8 | csr_number, 0);
        regs.insert(0b11 << 10 | 0b00 << 8 | csr_number, 0);

        // Supervisor CSRs.
        regs.insert(0b00 << 10 | 0b01 << 8 | csr_number, 0);
        regs.insert(0b01 << 10 | 0b01 << 8 | csr_number, 0);
        regs.insert(0b10 << 10 | 0b01 << 8 | csr_number, 0);
        regs.insert(0b11 << 10 | 0b01 << 8 | csr_number, 0);

        // Supervisor CSRs.
        regs.insert(0b00 << 10 | 0b10 << 8 | csr_number, 0);
        regs.insert(0b01 << 10 | 0b10 << 8 | csr_number, 0);
        regs.insert(0b10 << 10 | 0b10 << 8 | csr_number, 0);
        regs.insert(0b11 << 10 | 0b10 << 8 | csr_number, 0);

        // Machine CSRs.
        regs.insert(0b00 << 10 | 0b11 << 8 | csr_number, 0);
        regs.insert(0b01 << 10 | 0b11 << 8 | csr_number, 0);
        regs.insert(0b10 << 10 | 0b11 << 8 | csr_number, 0);
        regs.insert(0b11 << 10 | 0b11 << 8 | csr_number, 0);
    }

    pub fn read(&self, csr_address: u32) -> i64 {
        // TODO: Check a correct accessibility and a privileged level.
        let _accessibility = (csr_address & 0xc00) >> 10;
        let _privilege = (csr_address & 0x300) >> 8;

        *self.regs.get(&csr_address).expect("failed to read a csr")
    }

    pub fn write(&mut self, csr_address: u32, value: i64) {
        // TODO: Check a correct accessibility and a privileged level.
        let _accessibility = (csr_address & 0xc00) >> 10;
        let _privilege = (csr_address & 0x300) >> 8;

        let original = self.regs.get_mut(&csr_address).expect("failed to get a csr");
        *original = value;
    }
}
