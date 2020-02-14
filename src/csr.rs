use std::collections::HashMap;

pub struct Csr {
    regs: HashMap<u32, i64>,
}

impl Csr {
    pub fn new() -> Self {
        Self {
            regs: HashMap::new(),
        }
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
