//! The memory module contains the memory structure and implementation to read/write the memory.

/// The memory used by the emulator.
#[derive(Debug)]
pub struct Memory {
    pub dram: Vec<u8>,
}

impl Memory {
    /// Create a new `memory` object with default memory size (1048KB).
    pub fn new() -> Memory {
        Memory {
            // Default memory size is 1048KB.
            dram: vec![0; 1048 * 1000],
        }
    }

    /// Return the length of the memory.
    pub fn len(&self) -> usize {
        self.dram.len()
    }

    /// Set the binary in the memory.
    pub fn set_dram(&mut self, binary: Vec<u8>) {
        self.dram.splice(..binary.len(), binary.iter().cloned());
    }

    /// Write a byte to the memory.
    pub fn write8(&mut self, index: usize, val: u8) {
        self.dram[index] = val
    }

    /// Write 2 bytes to the memory.
    pub fn write16(&mut self, index: usize, val: u16) {
        self.dram[index] = (val & 0xFF) as u8;
        self.dram[index + 1] = ((val & 0xFF00) >> 8) as u8;
    }

    /// Write 4 bytes to the memory.
    pub fn write32(&mut self, index: usize, val: u32) {
        self.dram[index] = (val & 0xFF) as u8;
        self.dram[index + 1] = ((val & 0xFF00) >> 8) as u8;
        self.dram[index + 2] = ((val & 0xFF0000) >> 16) as u8;
        self.dram[index + 3] = ((val & 0xFF000000) >> 24) as u8;
    }

    /// Write 8 bytes to the memory.
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

    /// Read a byte from the memory.
    pub fn read8(&self, index: usize) -> u8 {
        self.dram[index]
    }

    /// Read 2 bytes from the memory.
    pub fn read16(&self, index: usize) -> u16 {
        // little endian
        return (self.dram[index] as u16) | ((self.dram[index + 1] as u16) << 8);
    }

    /// Read 4 bytes from the memory.
    pub fn read32(&self, index: usize) -> u32 {
        // little endian
        return (self.dram[index] as u32)
            | ((self.dram[index + 1] as u32) << 8)
            | ((self.dram[index + 2] as u32) << 16)
            | ((self.dram[index + 3] as u32) << 24);
    }

    /// Read 8 bytes from the memory.
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
