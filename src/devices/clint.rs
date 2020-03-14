//! The clint modules contains the core-local interruptor (CLINT). The CLINT
//! block holds memory-mapped control and status registers associated with
//! software and timer interrupts.

use crate::bus::CLINT_BASE;

/// The size of CLINT.
pub const CLINT_SIZE: usize = 0x10000;

/// The core-local interruptor (CLINT).
pub struct Clint {
    clint: [u8; CLINT_SIZE],
}

impl Clint {
    /// Create a new CLINT object.
    pub fn new() -> Self {
        Self {
            clint: [0; CLINT_SIZE],
        }
    }

    /// Read a byte from the CLINT.
    pub fn read8(&self, addr: usize) -> u8 {
        let index = addr - CLINT_BASE;
        self.clint[index]
    }

    /// Write a byte to the CLINT.
    pub fn write8(&mut self, addr: usize, val: u8) {
        let index = addr - CLINT_BASE;
        self.clint[index] = val
    }

    /// Read 8 bytes from the CLINT.
    pub fn read64(&self, addr: usize) -> u64 {
        let index = addr - CLINT_BASE;
        return (self.clint[index] as u64)
            | ((self.clint[index + 1] as u64) << 8)
            | ((self.clint[index + 2] as u64) << 16)
            | ((self.clint[index + 3] as u64) << 24)
            | ((self.clint[index + 4] as u64) << 32)
            | ((self.clint[index + 5] as u64) << 40)
            | ((self.clint[index + 6] as u64) << 48)
            | ((self.clint[index + 7] as u64) << 56);
    }

    /// Write 8 bytes from the CLINT.
    pub fn write64(&mut self, addr: usize, val: u64) {
        let index = addr - CLINT_BASE;
        self.clint[index] = (val & 0xFF) as u8;
        self.clint[index + 1] = ((val >> 8) & 0xFF) as u8;
        self.clint[index + 2] = ((val >> 16) & 0xFF) as u8;
        self.clint[index + 3] = ((val >> 24) & 0xFF) as u8;
        self.clint[index + 4] = ((val >> 32) & 0xFF) as u8;
        self.clint[index + 5] = ((val >> 40) & 0xFF) as u8;
        self.clint[index + 6] = ((val >> 48) & 0xFF) as u8;
        self.clint[index + 7] = ((val >> 56) & 0xFF) as u8;
    }
}
