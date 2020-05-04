//! The rom module contains the read-only memory structure and implementation to read the memory.

use crate::bus::MROM_BASE;

/// The read-only memory (ROM).
pub struct Rom {
    data: Vec<u8>,
}

impl Rom {
    /// Create a new `rom` object.
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Set the binary in the rom.
    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    /// Read a byte from the rom.
    pub fn read8(&self, addr: u64) -> u64 {
        let index = (addr - MROM_BASE) as usize;
        self.data[index] as u64
    }

    /// Read 2 bytes from the rom.
    pub fn read16(&self, addr: u64) -> u64 {
        let index = (addr - MROM_BASE) as usize;
        // little endian
        return (self.data[index] as u64) | ((self.data[index + 1] as u64) << 8);
    }

    /// Read 4 bytes from the rom.
    pub fn read32(&self, addr: u64) -> u64 {
        let index = (addr - MROM_BASE) as usize;
        // little endian
        return (self.data[index] as u64)
            | ((self.data[index + 1] as u64) << 8)
            | ((self.data[index + 2] as u64) << 16)
            | ((self.data[index + 3] as u64) << 24);
    }

    /// Read 8 bytes from the rom.
    pub fn read64(&self, addr: u64) -> u64 {
        let index = (addr - MROM_BASE) as usize;
        // little endian
        return (self.data[index] as u64)
            | ((self.data[index + 1] as u64) << 8)
            | ((self.data[index + 2] as u64) << 16)
            | ((self.data[index + 3] as u64) << 24)
            | ((self.data[index + 4] as u64) << 32)
            | ((self.data[index + 5] as u64) << 40)
            | ((self.data[index + 6] as u64) << 48)
            | ((self.data[index + 7] as u64) << 56);
    }
}
