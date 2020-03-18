//! The clint module contains the core-local interruptor (CLINT). The CLINT
//! block holds memory-mapped control and status registers associated with
//! software and timer interrupts. It generates per-hart software interrupts and timer.
//! The implementation compliant with the chapter 9 in "SiFive FU540-C000 Manual v1p0".
//! https://sifive.cdn.prismic.io/sifive%2F834354f0-08e6-423c-bf1f-0cb58ef14061_fu540-c000-v1.0.pdf

use crate::bus::CLINT_BASE;

/// The address of a mtimecmp register starts.
pub const CLINT_MTIMECMP_BASE: usize = CLINT_BASE + 0x4000;
/// The size of mtimecmp regsiters.
pub const CLINT_MTIMECMP_SIZE: usize = 0x28;
/// The address of a timer register.
pub const CLINT_MTIME: usize = CLINT_BASE + 0xbff8;

/// The core-local interruptor (CLINT).
/// 0x0000 msip hart 0
/// 0x0004 msip hart 1
/// 0x4000 mtimecmp hart 0 lo
/// 0x4004 mtimecmp hart 0 hi
/// 0x4008 mtimecmp hart 1 lo
/// 0x400c mtimecmp hart 1 hi
/// 0xbff8 mtime lo
/// 0xbffc mtime hi
pub struct Clint {
    /// 64-bit memory-mapped machine-mode timer compare registers (mtimecmp) mapped at 0x204000
    /// to 0x204020, which causes a timer interrupt to be posted when the mtime register contains
    /// a value greater than or equal to the value in the mtimecmp register.
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

    /// Increment the mtimer register.
    pub fn increment(&mut self) {
        self.mtime = self.mtime.wrapping_add(1);
    }

    /// Return true if an interrupt is pending and clear the `mtime` register if an interrupting
    /// is enable.
    pub fn is_interrupting(&mut self) -> bool {
        // Assume hart is 0.
        if self.mtime >= self.mtimecmps[0] {
            self.mtime = 0;
            true
        } else {
            false
        }
    }

    /// Read the content of a register from the CLINT.
    pub fn read(&self, addr: usize) -> u64 {
        if CLINT_MTIMECMP_BASE <= addr && addr < CLINT_MTIMECMP_BASE + CLINT_MTIMECMP_SIZE {
            let index = (addr - CLINT_MTIMECMP_BASE) / 8;
            return self.mtimecmps[index];
        } else if addr == CLINT_MTIME {
            return self.mtime;
        }
        0
    }

    /// Write the content of a register from the CLINT.
    pub fn write(&mut self, addr: usize, val: u64) {
        if CLINT_MTIMECMP_BASE <= addr && addr < CLINT_MTIMECMP_BASE + CLINT_MTIMECMP_SIZE {
            let index = (addr - CLINT_MTIMECMP_BASE) / 8;
            self.mtimecmps[index] = val;
        } else if addr == CLINT_MTIME {
            self.mtime = val;
        }
    }
}
