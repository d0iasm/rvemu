//! The memory module contains the memory structure and implementation to read/write the memory.

/// The memory used by the emulator.
#[derive(Debug)]
pub struct Memory {
    pub dram: Vec<u8>,
    code_size: usize,
}

impl Memory {
    /// Create a new `memory` object with default memory size (1048KB).
    pub fn new() -> Memory {
        Self {
            // Default memory size is 128MiB.
            dram: vec![0; 1024 * 1024 * 128],
            code_size: 0,
        }
    }

    /// Return the code size in the memory.
    pub fn size(&self) -> usize {
        self.code_size
    }

    /// Set the binary in the memory.
    pub fn set_dram(&mut self, binary: Vec<u8>) {
        self.code_size = binary.len();
        self.dram.splice(..binary.len(), binary.iter().cloned());
    }

    /// Write a byte to the memory.
    pub fn write8(&mut self, addr: usize, val: u8) {
        self.dram[addr] = val
    }

    /// Write 2 bytes to the memory.
    pub fn write16(&mut self, addr: usize, val: u16) {
        self.dram[addr] = (val & 0xFF) as u8;
        self.dram[addr + 1] = ((val >> 8) & 0xFF) as u8;
    }

    /// Write 4 bytes to the memory.
    pub fn write32(&mut self, addr: usize, val: u32) {
        self.dram[addr] = (val & 0xFF) as u8;
        self.dram[addr + 1] = ((val >> 8) & 0xFF) as u8;
        self.dram[addr + 2] = ((val >> 16) & 0xFF) as u8;
        self.dram[addr + 3] = ((val >> 24) & 0xFF) as u8;
    }

    /// Write 8 bytes to the memory.
    pub fn write64(&mut self, addr: usize, val: u64) {
        self.dram[addr] = (val & 0xFF) as u8;
        self.dram[addr + 1] = ((val >> 8) & 0xFF) as u8;
        self.dram[addr + 2] = ((val >> 16) & 0xFF) as u8;
        self.dram[addr + 3] = ((val >> 24) & 0xFF) as u8;
        self.dram[addr + 4] = ((val >> 32) & 0xFF) as u8;
        self.dram[addr + 5] = ((val >> 40) & 0xFF) as u8;
        self.dram[addr + 6] = ((val >> 48) & 0xFF) as u8;
        self.dram[addr + 7] = ((val >> 56) & 0xFF) as u8;
    }

    /// Read a byte from the memory.
    pub fn read8(&self, addr: usize) -> u8 {
        self.dram[addr]
    }

    /// Read 2 bytes from the memory.
    pub fn read16(&self, addr: usize) -> u16 {
        // little endian
        return (self.dram[addr] as u16) | ((self.dram[addr + 1] as u16) << 8);
    }

    /// Read 4 bytes from the memory.
    pub fn read32(&self, addr: usize) -> u32 {
        // little endian
        return (self.dram[addr] as u32)
            | ((self.dram[addr + 1] as u32) << 8)
            | ((self.dram[addr + 2] as u32) << 16)
            | ((self.dram[addr + 3] as u32) << 24);
    }

    /// Read 8 bytes from the memory.
    pub fn read64(&self, addr: usize) -> u64 {
        // little endian
        return (self.dram[addr] as u64)
            | ((self.dram[addr + 1] as u64) << 8)
            | ((self.dram[addr + 2] as u64) << 16)
            | ((self.dram[addr + 3] as u64) << 24)
            | ((self.dram[addr + 4] as u64) << 32)
            | ((self.dram[addr + 5] as u64) << 40)
            | ((self.dram[addr + 6] as u64) << 48)
            | ((self.dram[addr + 7] as u64) << 56);
    }
}
