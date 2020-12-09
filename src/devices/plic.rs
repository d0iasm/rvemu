//! The plic module contains the platform-level interrupt controller (PLIC).
//! The plic connects all external interrupts in the system to all hart
//! contexts in the system, via the external interrupt source in each hart.
//! It's the global interrupt controller in a RISC-V system.

// Reference:
// "SiFive Interrupt Cookbook Version 1.0"
// https://sifive.cdn.prismic.io/sifive/0d163928-2128-42be-a75a-464df65e04e0_sifive-interrupt-cookbook.pdf
// "RISC-V Platform-Level Interrupt Controller Specification"
// https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc
//
// QEMU SiFive PLIC used in the virt machine:
// - https://github.com/qemu/qemu/blob/master/hw/intc/sifive_plic.c
// - https://github.com/qemu/qemu/blob/master/include/hw/intc/sifive_plic.h

use crate::bus::PLIC_BASE;
use crate::devices::{uart::UART_IRQ, virtio::VIRTIO_IRQ};
use crate::exception::Exception;

/// The address of interrupt source priority.
pub const PLIC_SOURCE_PRIORITY: u64 = PLIC_BASE;
/// The address of interrupt pending bits.
pub const PLIC_PENDING: u64 = PLIC_BASE + 0x1000;
/// The address of the regsiters to enable interrupts for M-mode.
pub const PLIC_MENABLE: u64 = PLIC_BASE + 0x2000;
/// The address of the regsiters to enable interrupts for S-mode.
pub const PLIC_SENABLE: u64 = PLIC_BASE + 0x2080;
/// The address of the registers to set a priority for M-mode.
pub const PLIC_MPRIORITY: u64 = PLIC_BASE + 0x200000;
/// The address of the claim/complete registers for M-mode.
pub const PLIC_MCLAIM: u64 = PLIC_BASE + 0x200004;
/// The address of the registers to set a priority for S-mode.
pub const PLIC_SPRIORITY: u64 = PLIC_BASE + 0x201000;
/// The address of the claim/complete registers for S-mode.
pub const PLIC_SCLAIM: u64 = PLIC_BASE + 0x201004;

/// The platform-level-interrupt controller (PLIC).
pub struct Plic {
    /// Priority register.
    /// The QEMU virt machine supports 7 levels of priority. A priority value of 0 is
    /// reserved to mean "never interrupt" and effectively disables the interrupt. Priority 1 is
    /// the lowest active priority, and priority 7 is the highest.
    /// The xv6 uses only 2 devices, uart and virtio, so the array contains 2 elements for now.
    priority: [u32; 2],
    /// Interrupt pending bits. If bit 1 is set, a global interrupt 1 is pending.
    /// A pending bit in the PLIC core can be cleared by setting the associated enable bit then performing a claim.
    pending: u32,
    /// S-mode enables registers that each global interrupt can be enabled by setting the
    /// corresponding to.
    /// 0 means no global interrupts exist. If bit 1 is set, a global interrupt 1 is enabled.
    /// The number of `hart` is expected under 5 for now.
    senable: [u32; 5],
    /// M-mode threshold registers for an interrupt priority threshold.
    /// The number of `hart` is expected under 5 for now.
    mpriority: [u32; 5],
    /// S-mode threshold registers for an interrupt priority threshold.
    /// The number of `hart` is expected under 5 for now.
    spriority: [u32; 5],
    /// S-mode claim/complete register, which returns the ID of the highest-priority pending
    /// interrupt or zero if there is no pending interrupt. A successful claim also atomically clears the
    /// corresponding pending bit on the interrupt source.
    sclaim: [u32; 5],
}

impl Plic {
    /// Create a new PLIC object.
    pub fn new() -> Self {
        Self {
            priority: [0; 2],
            pending: 0,
            senable: [0; 5],
            mpriority: [0; 5],
            spriority: [0; 5],
            sclaim: [0; 5],
        }
    }

    /// Read 4 bytes from the PLIC only if the address is valid. Otherwise, returns 0.
    pub fn read32(&self, addr: u64) -> Result<u64, Exception> {
        if PLIC_SOURCE_PRIORITY <= addr && addr <= PLIC_SOURCE_PRIORITY + 0x000ffc {
            // TODO: handle other source devices.
            return Ok(self.priority[0] as u64);
        }
        if addr == PLIC_PENDING {
            return Ok(self.pending as u64);
        }
        if PLIC_SENABLE <= addr && addr <= PLIC_SENABLE + 0x100 * self.senable.len() as u64 {
            let index = (addr - PLIC_SENABLE) / 0x100;

            return Ok(self.senable[index as usize] as u64);
        }
        if PLIC_MPRIORITY <= addr && addr <= PLIC_MPRIORITY + 0x2000 * self.spriority.len() as u64 {
            let index = (addr - PLIC_MPRIORITY) / 0x2000;
            return Ok(self.mpriority[index as usize] as u64);
        }
        if PLIC_SPRIORITY <= addr && addr <= PLIC_SPRIORITY + 0x2000 * self.spriority.len() as u64 {
            let index = (addr - PLIC_SPRIORITY) / 0x2000;
            return Ok(self.spriority[index as usize] as u64);
        }
        if PLIC_SCLAIM <= addr && addr <= PLIC_SCLAIM + 0x2000 * self.sclaim.len() as u64 {
            let index = (addr - PLIC_SCLAIM) / 0x2000;
            return Ok(self.sclaim[index as usize] as u64);
        }
        println!("plic read: {:#x}", addr);
        Err(Exception::LoadAccessFault)
    }

    /// Write 4 bytes to the PLIC only if the address is valid.
    pub fn write32(&mut self, addr: u64, val: u32) -> Result<(), Exception> {
        if PLIC_SOURCE_PRIORITY <= addr && addr <= PLIC_BASE + 0x000FFC {
            // TODO: handle other source devices.
            self.priority[0] = val;
            return Ok(());
        }
        if addr == PLIC_PENDING {
            self.pending = val;
            return Ok(());
        }
        if PLIC_SENABLE <= addr && addr <= PLIC_SENABLE + 0x100 * self.senable.len() as u64 {
            let index = (addr - PLIC_SENABLE) / 0x100;

            self.senable[index as usize] = val;
            return Ok(());
        }
        if PLIC_MPRIORITY <= addr && addr <= PLIC_MPRIORITY + 0x2000 * self.spriority.len() as u64 {
            let index = (addr - PLIC_MPRIORITY) / 0x2000;
            self.mpriority[index as usize] = val;
            return Ok(());
        }
        if PLIC_SPRIORITY <= addr && addr <= PLIC_SPRIORITY + 0x2000 * self.spriority.len() as u64 {
            let index = (addr - PLIC_SPRIORITY) / 0x2000;
            self.spriority[index as usize] = val;
            return Ok(());
        }
        if PLIC_SCLAIM <= addr && addr <= PLIC_SCLAIM + 0x2000 * self.sclaim.len() as u64 {
            let index = (addr - PLIC_SCLAIM) / 0x2000;
            self.sclaim[index as usize] = val;
            return Ok(());
        }
        println!("plic write: {:#x}", PLIC_BASE + UART_IRQ * 4);
        println!("plic write: {:#x} -> {:#x}", addr, val);
        Err(Exception::StoreAMOAccessFault)
    }
}
