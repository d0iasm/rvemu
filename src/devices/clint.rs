//! The clint modules contains the core-local interruptor (CLINT). The CLINT
//! block holds memory-mapped control and status registers associated with
//! software and timer interrupts. It generates per-hart software interrupts and timer.
//! The implementation compliant with "SiFive FU540-C000 Manual v1p0".
//! https://sifive.cdn.prismic.io/sifive%2F834354f0-08e6-423c-bf1f-0cb58ef14061_fu540-c000-v1.0.pdf

use crate::bus::CLINT_BASE;

/// The size of CLINT.
pub const CLINT_SIZE: usize = 0x10000;

/// The address of a mtimecmp register starts.
pub const CLINT_MTIMECMP_BASE: usize = CLINT_BASE + 0x4000;
/// The address of a mtimecmp register ends.
pub const CLINT_MTIMECMP_END: usize = CLINT_BASE + 0x4000 + 0x20;
/// The address of a timer register.
pub const CLINT_MTIME: usize = CLINT_BASE + 0xbff8;

/// The core-local interruptor (CLINT).
pub struct Clint {
    /// Mtimecmp registers mapped at 0x204000 to 0x204020.
    mtimecmps: [u64; 5],
    /// Timer register mapped at 0x20bff8.
    mtime: u64,
}

impl Clint {
    /// Create a new CLINT object.
    pub fn new() -> Self {
        Self {
            mtimecmps: [0; 5],
            mtime: 0,
        }
    }

    /// Read the content of a register from the CLINT.
    pub fn read(&self, addr: usize) -> u64 {
        if CLINT_MTIMECMP_BASE <= addr && addr <= CLINT_MTIMECMP_END {
            let index = (addr - CLINT_MTIMECMP_BASE) / 8;
            return self.mtimecmps[index];
        } else if addr == CLINT_MTIME {
            return self.mtime;
        }
        0
    }

    /// Write the content of a register from the CLINT.
    pub fn write(&mut self, addr: usize, value: u64) {
        if CLINT_MTIMECMP_BASE <= addr && addr <= CLINT_MTIMECMP_END {
            let index = (addr - CLINT_MTIMECMP_BASE) / 8;
            self.mtimecmps[index] = value;
        } else if addr == CLINT_MTIME {
            self.mtime = value;
        }
    }
}
