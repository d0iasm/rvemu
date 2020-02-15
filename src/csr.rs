use std::collections::HashMap;

pub struct Csr {
    regs: HashMap<u32, i64>,
}

impl Csr {
    pub fn new() -> Self {
        let mut regs = HashMap::new();
        regs.insert(0x301, 0); // misa: Machine interrupt-enable register.

        Self { regs }
    }

    pub fn read(&self, csr_address: u32) -> i64 {
        let accessibility = (csr_address & 0xc00) >> 10;
        let privilege = (csr_address & 0x300) >> 8;
        let number = csr_address & 0xff;

        *self.regs.get(&number).expect("failed to read a csr")
    }

    pub fn write(&mut self, csr_address: u32, value: i64) {
        let accessibility = (csr_address & 0xc00) >> 10;
        let privilege = (csr_address & 0x300) >> 8;
        let number = csr_address & 0xff;

        let original = self.regs.get_mut(&number).expect("failed to get a csr");
        *original = value;
    }
}
