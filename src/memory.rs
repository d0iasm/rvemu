pub struct Memory {
    pub dram: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            // Default memory size is 2048MB.
            dram: vec![0; 2048 * 1000 * 1000],
        }
    }

    pub fn len(&self) -> usize {
        self.dram.len()
    }

    pub fn set_binary(&mut self, binary: Vec<u8>) {
        self.dram = binary;
    }

    pub fn write8(&mut self, index: usize, val: u8) {
        self.dram[index] = val
    }

    pub fn write16(&mut self, index: usize, val: u16) {
        self.dram[index] = (val & 0xFF) as u8;
        self.dram[index + 1] = ((val & 0xFF00) >> 8) as u8;
    }

    pub fn write32(&mut self, index: usize, val: u32) {
        self.dram[index] = (val & 0xFF) as u8;
        self.dram[index + 1] = ((val & 0xFF00) >> 8) as u8;
        self.dram[index + 2] = ((val & 0xFF0000) >> 16) as u8;
        self.dram[index + 3] = ((val & 0xFF000000) >> 24) as u8;
    }

    pub fn write64(&mut self, index: usize, val: u64) {
        self.dram[index] = (val & 0xFF) as u8;
        self.dram[index + 1] = ((val & 0xFF00) >> 8) as u8;
        self.dram[index + 2] = ((val & 0xFF0000) >> 16) as u8;
        self.dram[index + 3] = ((val & 0xFF000000) >> 24) as u8;
        self.dram[index + 4] = ((val & 0xFF00000000) >> 32) as u8;
        self.dram[index + 5] = ((val & 0xFF0000000000) >> 40) as u8;
        self.dram[index + 6] = ((val & 0xFF000000000000) >> 48) as u8;
        self.dram[index + 7] = ((val & 0xFF00000000000000) >> 56) as u8;
    }

    pub fn read8(&self, index: usize) -> u8 {
        self.dram[index]
    }

    pub fn read16(&self, index: usize) -> u16 {
        // little endian
        return (self.dram[index] as u16) | ((self.dram[index + 1] as u16) << 8);
    }

    pub fn read32(&self, index: usize) -> u32 {
        // little endian
        return (self.dram[index] as u32)
            | ((self.dram[index + 1] as u32) << 8)
            | ((self.dram[index + 2] as u32) << 16)
            | ((self.dram[index + 3] as u32) << 24);
    }

    pub fn read64(&self, index: usize) -> u64 {
        // little endian
        return (self.dram[index] as u64)
            | ((self.dram[index + 1] as u64) << 8)
            | ((self.dram[index + 2] as u64) << 16)
            | ((self.dram[index + 3] as u64) << 24)
            | ((self.dram[index + 4] as u64) << 32)
            | ((self.dram[index + 5] as u64) << 40)
            | ((self.dram[index + 6] as u64) << 48)
            | ((self.dram[index + 7] as u64) << 56);
    }
}
