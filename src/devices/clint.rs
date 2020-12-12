//! The clint module contains the core-local interruptor (CLINT). The CLINT block holds
//! memory-mapped control and status registers associated with software and timer interrupts. It
//! generates per-hart software interrupts and timer.

// Reference:
// "SiFive Interrupt Cookbook Version 1.0"
// https://sifive.cdn.prismic.io/sifive/0d163928-2128-42be-a75a-464df65e04e0_sifive-interrupt-cookbook.pdf
//
// QEMU SiFive CLINT used in the virt machine:
// - https://github.com/qemu/qemu/blob/master/hw/intc/sifive_clint.c
// - https://github.com/qemu/qemu/blob/master/include/hw/intc/sifive_clint.h

use crate::bus::CLINT_BASE;
use crate::csr::{State, MIP};
use crate::exception::Exception;

/// The address of a msip register starts. A msip is a machine mode software interrupt pending
/// register, used to assert a software interrupt for a CPU.
pub const CLINT_MSIP: u64 = CLINT_BASE;
/// The address of a mtimecmp register starts. A mtimecmp is a memory mapped machine mode timer
/// compare register, used to trigger an interrupt when mtimecmp is greater than or equal to mtime.
pub const CLINT_MTIMECMP: u64 = CLINT_BASE + 0x4000;
/// The address of a timer register. A mtime is a machine mode timer register which runs at a
/// constant frequency.
pub const CLINT_MTIME: u64 = CLINT_BASE + 0xbff8;

/// The core-local interruptor (CLINT).
/// 0x0000 msip hart 0
/// 0x4000 mtimecmp hart 0 lo
/// 0x4004 mtimecmp hart 0 hi
/// 0xbff8 mtime lo
/// 0xbffc mtime hi
pub struct Clint {
    /// Machine mode software interrupt pending register, used to assert a software interrupt for
    /// a CPU.
    msip: u32,
    /// Memory mapped machine mode timer compare register, used to trigger an interrupt when
    /// mtimecmp is greater than or equal to mtime. There is an mtimecmp dedicated to each CPU.
    mtimecmp: u64,
    /// Machine mode timer register which runs at a constant frequency.
    mtime: u64,
}

impl Clint {
    /// Create a new CLINT object.
    pub fn new() -> Self {
        Self {
            msip: 0,
            mtimecmp: 0,
            mtime: 0,
        }
    }

    /// Increment the mtimer register. It's not a real-time value. The MTIP bit (MIP, 7) is enabled
    /// when `mtime` is greater than or equal to `mtimecmp`.
    pub fn increment(&mut self, state: &mut State) {
        self.mtime = self.mtime.wrapping_add(1);

        // Clear the MSIP bit (MIP, 3).
        state.write(MIP, state.read(MIP) & !(1 << 3));
        if (self.msip & 1) != 0 {
            // Enable the MSIP bit (MIP, 3).
            state.write(MIP, state.read(MIP) | (1 << 3));
        }

        // Clear the MTIP bit (MIP, 7).
        state.write(MIP, state.read(MIP) & !(1 << 7));
        if self.mtime >= self.mtimecmp {
            // Enable the MTIP bit (MIP, 7).
            state.write(MIP, state.read(MIP) | (1 << 7));
            self.mtime = 0;
        }
    }

    /// Return true if an interrupt is pending and clear the `mtime` register if an interrupting
    /// is enable.
    pub fn is_interrupting(&mut self) -> bool {
        if self.mtime >= self.mtimecmp {
            self.mtime = 0;
            true
        } else {
            false
        }
    }

    /// Read `size`-bit data from a register located at `addr` in CLINT.
    pub fn read(&self, addr: u64, size: u8) -> Result<u64, Exception> {
        // TODO: Access to addr + 1/2/3 bytes
        let value = match addr {
            CLINT_MSIP => self.msip as u64,
            CLINT_MTIMECMP => self.mtimecmp,
            CLINT_MTIME => self.mtime,
            _ => return Err(Exception::LoadAccessFault),
        };

        match size {
            8 => Ok(value & 0xff),
            16 => Ok(value & 0xffff),
            32 => Ok(value & 0xffffffff),
            64 => Ok(value),
            _ => Err(Exception::LoadAccessFault),
        }
    }

    /// Write `size`-bit data to a register located at `addr` in CLINT.
    pub fn write(&mut self, addr: u64, value: u64, size: u8) -> Result<(), Exception> {
        // TODO: Access to addr + 1/2/3 bytes
        let v = match size {
            8 => value & 0xff,
            16 => value & 0xffff,
            32 => value & 0xffffffff,
            64 => value,
            _ => return Err(Exception::StoreAMOAccessFault),
        };

        match addr {
            CLINT_MSIP => self.msip = v as u32,
            CLINT_MTIMECMP => self.mtimecmp = v,
            CLINT_MTIME => self.mtime = v,
            _ => return Err(Exception::StoreAMOAccessFault),
        }

        Ok(())
    }
}
