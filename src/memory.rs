//! The memory module contains the memory structure and implementation to read/write the memory.

use crate::bus::DRAM_BASE;

/// Default memory size (1GiB).
pub const MEMORY_SIZE: u64 = 1024 * 1024 * 1024;

/// The memory used by the emulator.
#[derive(Debug)]
pub struct Memory {
    pub dram: Vec<u8>,
    code_size: u64,
}

impl Memory {
    /// Create a new memory object with default memory size.
    pub fn new() -> Memory {
        Self {
            dram: vec![0; MEMORY_SIZE as usize],
            code_size: 0,
        }
    }

    /// Return the code size in the memory.
    pub fn size(&self) -> u64 {
        self.code_size
    }

    /// Set the binary in the memory.
    pub fn set_dram(&mut self, binary: Vec<u8>) {
        self.code_size = binary.len() as u64;
        self.dram.splice(..binary.len(), binary.iter().cloned());
    }

    /// Write a byte to the memory.
    pub fn write8(&mut self, addr: u64, val: u64) {
        let index = (addr - DRAM_BASE) as usize;
        self.dram[index] = val as u8
    }

    /// Write 2 bytes to the memory.
    pub fn write16(&mut self, addr: u64, val: u64) {
        let index = (addr - DRAM_BASE) as usize;
        self.dram[index] = (val & 0xFF) as u8;
        self.dram[index + 1] = ((val >> 8) & 0xFF) as u8;
    }

    /// Write 4 bytes to the memory.
    pub fn write32(&mut self, addr: u64, val: u64) {
        let index = (addr - DRAM_BASE) as usize;
        self.dram[index] = (val & 0xFF) as u8;
        self.dram[index + 1] = ((val >> 8) & 0xFF) as u8;
        self.dram[index + 2] = ((val >> 16) & 0xFF) as u8;
        self.dram[index + 3] = ((val >> 24) & 0xFF) as u8;
    }

    /// Write 8 bytes to the memory.
    pub fn write64(&mut self, addr: u64, val: u64) {
        let index = (addr - DRAM_BASE) as usize;
        self.dram[index] = (val & 0xFF) as u8;
        self.dram[index + 1] = ((val >> 8) & 0xFF) as u8;
        self.dram[index + 2] = ((val >> 16) & 0xFF) as u8;
        self.dram[index + 3] = ((val >> 24) & 0xFF) as u8;
        self.dram[index + 4] = ((val >> 32) & 0xFF) as u8;
        self.dram[index + 5] = ((val >> 40) & 0xFF) as u8;
        self.dram[index + 6] = ((val >> 48) & 0xFF) as u8;
        self.dram[index + 7] = ((val >> 56) & 0xFF) as u8;
    }

    /// Read a byte from the memory.
    pub fn read8(&self, addr: u64) -> u64 {
        let index = (addr - DRAM_BASE) as usize;
        self.dram[index] as u64
    }

    /// Read 2 bytes from the memory.
    pub fn read16(&self, addr: u64) -> u64 {
        let index = (addr - DRAM_BASE) as usize;
        // little endian
        return (self.dram[index] as u64) | ((self.dram[index + 1] as u64) << 8);
    }

    /// Read 4 bytes from the memory.
    pub fn read32(&self, addr: u64) -> u64 {
        let index = (addr - DRAM_BASE) as usize;
        // little endian
        return (self.dram[index] as u64)
            | ((self.dram[index + 1] as u64) << 8)
            | ((self.dram[index + 2] as u64) << 16)
            | ((self.dram[index + 3] as u64) << 24);
    }

    /// Read 8 bytes from the memory.
    pub fn read64(&self, addr: u64) -> u64 {
        let index = (addr - DRAM_BASE) as usize;
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
