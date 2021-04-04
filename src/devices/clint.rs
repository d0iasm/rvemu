//! The clint module contains the core-local interruptor (CLINT). The CLINT block holds
//! memory-mapped control and status registers associated with software and timer interrupts. It
//! generates per-hart software interrupts and timer.

// Reference:
// "SiFive Interrupt Cookbook Version 1.0"
// https://sifive.cdn.prismic.io/sifive/0d163928-2128-42be-a75a-464df65e04e0_sifive-interrupt-cookbook.pdf
// Chapter 8 Core-Local Interruptor (CLINT) in "U74 Core Complex Manual"
// https://sifive.cdn.prismic.io/sifive/132ebfe4-e7eb-4274-b456-d79835e10d8d_sifive_U74_rtl_full_20G1.03.00_manual.pdf
//
// QEMU SiFive CLINT used in the virt machine:
// - https://github.com/qemu/qemu/blob/master/hw/intc/sifive_clint.c
// - https://github.com/qemu/qemu/blob/master/include/hw/intc/sifive_clint.h

use crate::bus::CLINT_BASE;
use crate::cpu::{BYTE, DOUBLEWORD, HALFWORD, WORD};
use crate::csr::{State, MIP, MSIP_BIT, MTIP_BIT};
use crate::exception::Exception;

/// The address that a msip register starts. A msip is a machine mode software interrupt pending
/// register, used to assert a software interrupt for a CPU.
const MSIP: u64 = CLINT_BASE;
/// The address that a msip register ends. `msip` is a 4-byte register.
const MSIP_END: u64 = MSIP + 0x4;

/// The address that a mtimecmp register starts. A mtimecmp is a memory mapped machine mode timer
/// compare register, used to trigger an interrupt when mtimecmp is greater than or equal to mtime.
const MTIMECMP: u64 = CLINT_BASE + 0x4000;
/// The address that a mtimecmp register ends. `mtimecmp` is a 8-byte register.
const MTIMECMP_END: u64 = MTIMECMP + 0x8;

/// The address that a timer register starts. A mtime is a machine mode timer register which runs
/// at a constant frequency.
const MTIME: u64 = CLINT_BASE + 0xbff8;
/// The address that a timer register ends. `mtime` is a 8-byte register.
const MTIME_END: u64 = MTIME + 0x8;

/// The core-local interruptor (CLINT).
/// 0x0000 msip for hart 0 (4 bytes)
/// 0x4000 mtimecmp for hart 0 (8 bytes)
/// 0xbff8 mtime (8 bytes)
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
        // Sync TIME csr.
        //state.write(TIME, self.mtime);

        if (self.msip & 1) != 0 {
            // Enable the MSIP bit (MIP, 3).
            state.write(MIP, state.read(MIP) | MSIP_BIT);
        }

        // 3.1.10 Machine Timer Registers (mtime and mtimecmp)
        // "The interrupt remains posted until mtimecmp becomes greater than mtime (typically as a
        // result of writing mtimecmp)."
        if self.mtimecmp > self.mtime {
            // Clear the MTIP bit (MIP, 7).
            state.write(MIP, state.read(MIP) & !MTIP_BIT);
        }

        // 3.1.10 Machine Timer Registers (mtime and mtimecmp)
        // "A timer interrupt becomes pending whenever mtime contains a value greater than or equal
        // to mtimecmp, treating the values as unsigned integers."
        if self.mtime >= self.mtimecmp {
            // Enable the MTIP bit (MIP, 7).
            state.write(MIP, state.read(MIP) | MTIP_BIT);
        }
    }

    /// Load `size`-bit data from a register located at `addr` in CLINT.
    pub fn read(&self, addr: u64, size: u8) -> Result<u64, Exception> {
        // `reg` is the value of a target register in CLINT and `offset` is the byte of the start
        // position in the register.
        let (reg, offset) = match addr {
            MSIP..=MSIP_END => (self.msip as u64, addr - MSIP),
            MTIMECMP..=MTIMECMP_END => (self.mtimecmp, addr - MTIMECMP),
            MTIME..=MTIME_END => (self.mtime, addr - MTIME),
            _ => return Err(Exception::LoadAccessFault),
        };

        match size {
            BYTE => Ok((reg >> (offset * 8)) & 0xff),
            HALFWORD => Ok((reg >> (offset * 8)) & 0xffff),
            WORD => Ok((reg >> (offset * 8)) & 0xffffffff),
            DOUBLEWORD => Ok(reg),
            _ => return Err(Exception::LoadAccessFault),
        }
    }

    /// Store `size`-bit data to a register located at `addr` in CLINT.
    pub fn write(&mut self, addr: u64, value: u64, size: u8) -> Result<(), Exception> {
        // `reg` is the value of a target register in CLINT and `offset` is the byte of the start
        // position in the register.
        let (mut reg, offset) = match addr {
            MSIP..=MSIP_END => (self.msip as u64, addr - MSIP),
            MTIMECMP..=MTIMECMP_END => (self.mtimecmp, addr - MTIMECMP),
            MTIME..=MTIME_END => (self.mtime, addr - MTIME),
            _ => return Err(Exception::StoreAMOAccessFault),
        };

        // Calculate the new value of the target register based on `size` and `offset`.
        match size {
            BYTE => {
                // Clear the target byte.
                reg = reg & (!(0xff << (offset * 8)));
                // Set the new `value` to the target byte.
                reg = reg | ((value & 0xff) << (offset * 8));
            }
            HALFWORD => {
                reg = reg & (!(0xffff << (offset * 8)));
                reg = reg | ((value & 0xffff) << (offset * 8));
            }
            WORD => {
                reg = reg & (!(0xffffffff << (offset * 8)));
                reg = reg | ((value & 0xffffffff) << (offset * 8));
            }
            DOUBLEWORD => {
                reg = value;
            }
            _ => return Err(Exception::StoreAMOAccessFault),
        }

        // Store the new value to the target register.
        match addr {
            MSIP..=MSIP_END => self.msip = reg as u32,
            MTIMECMP..=MTIMECMP_END => self.mtimecmp = reg,
            MTIME..=MTIME_END => self.mtime = reg,
            _ => return Err(Exception::StoreAMOAccessFault),
        }

        Ok(())
    }
}
