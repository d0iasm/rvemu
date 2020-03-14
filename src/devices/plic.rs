//! The plic modules contains the platform-level interrupt controller (PLIC).
//! The PLIC connects all external interrupts in the system to all hart
//! contexts in the system, via the external interrupt source in each hart. It's the global interrupt controller in a RISC-V system.

use crate::bus::PLIC_BASE;

/// The size of PLIC.
pub const PLIC_SIZE: usize = 0x4000000;

/// Return the address of PLIC_SCLAIM.
pub fn plic_sclaim(hart: usize) -> usize {
    PLIC_BASE + 0x201004 + hart * 0x2000
}

pub struct Plic {
    // TODO: Rewrite. Should have flags instead of an array.
    plic: [u8; PLIC_SIZE],
}

impl Plic {
    /// Create a new PLIC object.
    pub fn new() -> Self {
        Self {
            plic: [0; PLIC_SIZE],
        }
    }

    /// Read 4 bytes from the PLIC.
    pub fn read32(&self, addr: usize) -> u32 {
        let index = addr - PLIC_BASE;
        return (self.plic[index] as u32)
            | ((self.plic[index + 1] as u32) << 8)
            | ((self.plic[index + 2] as u32) << 16)
            | ((self.plic[index + 3] as u32) << 24);
    }

    /// Write 4 bytes to the PLIC.
    pub fn write32(&mut self, addr: usize, val: u32) {
        let index = addr - PLIC_BASE;
        self.plic[index] = (val & 0xFF) as u8;
        self.plic[index + 1] = ((val >> 8) & 0xFF) as u8;
        self.plic[index + 2] = ((val >> 16) & 0xFF) as u8;
        self.plic[index + 3] = ((val >> 24) & 0xFF) as u8;
    }
}
