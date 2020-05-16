//! The plic module contains the platform-level interrupt controller (PLIC).
//! The plic connects all external interrupts in the system to all hart
//! contexts in the system, via the external interrupt source in each hart.
//! It's the global interrupt controller in a RISC-V system.

// Good documents for understanding PLIC:
// - "SiFive FU540-C000 Manual v1p0":
// https://sifive.cdn.prismic.io/sifive%2F834354f0-08e6-423c-bf1f-0cb58ef14061_fu540-c000-v1.0.pdf
// - "SiFive Interrupt Cookbook Version 1.0":
// https://sifive.cdn.prismic.io/sifive/0d163928-2128-42be-a75a-464df65e04e0_sifive-interrupt-cookbook.pdf

use crate::bus::PLIC_BASE;
use crate::devices::{uart::UART_IRQ, virtio::VIRTIO_IRQ};

/// The address of interrupt pending bits.
pub const PLIC_PENDING: u64 = PLIC_BASE + 0x1000;
/// The address of the regsiters to enable interrupts for M-mode.
pub const PLIC_MENABLE: u64 = PLIC_BASE + 0x2000;
/// The address of the regsiters to enable interrupts for S-mode.
pub const PLIC_SENABLE: u64 = PLIC_BASE + 0x2080;
/// The address of the registers to set a priority for M-mode.
pub const PLIC_MPRIORITY: u64 = PLIC_BASE + 0x200000;
/// The address of the registers to set a priority for S-mode.
pub const PLIC_SPRIORITY: u64 = PLIC_BASE + 0x201000;
/// The address of the claim/complete registers for M-mode.
pub const PLIC_MCLAIM: u64 = PLIC_BASE + 0x200004;
/// The address of the claim/complete registers for S-mode.
pub const PLIC_SCLAIM: u64 = PLIC_BASE + 0x201004;

/// The platform-level-interrupt controller (PLIC).
pub struct Plic {
    /// Priority register.
    /// The QEMU virt machine supports 7 levels of priority. A priority value of 0 is
    /// reserved to mean "never interrupt" and effectively disables the interrupt. Priority 1 is
    /// the lowest active priority, and priority 7 is the highest.
    /// The xv6 uses only 2 devices, uart and virtio, so the array contains 2 elements for now.
    priorities: [u32; 2],
    /// Interrupt pending bits. If bit 1 is set, a global interrupt 1 is pending.
    /// A pending bit in the PLIC core can be cleared by setting the associated enable bit then performing a claim.
    pending: u32,
    /// S-mode enables registers that each global interrupt can be enabled by setting the
    /// corresponding to.
    /// 0 means no global interrupts exist. If bit 1 is set, a global interrupt 1 is enabled.
    /// The number of `hart` is expected under 5 for now.
    senables: [u32; 5],
    /// S-mode threshold registers for an interrupt priority threshold.
    /// The number of `hart` is expected under 5 for now.
    spriorities: [u32; 5],
    /// S-mode claim/complete register, which returns the ID of the highest-priority pending
    /// interrupt or zero if there is no pending interrupt. A successful claim also atomically clears the
    /// corresponding pending bit on the interrupt source.
    sclaims: [u32; 5],
}

impl Plic {
    /// Create a new PLIC object.
    pub fn new() -> Self {
        Self {
            priorities: [0; 2],
            pending: 0,
            senables: [0; 5],
            spriorities: [0; 5],
            sclaims: [0; 5],
        }
    }

    /// Read 4 bytes from the PLIC only if the address is valid. Otherwise, returns 0.
    pub fn read(&self, addr: u64) -> u32 {
        // TODO: This `if` statement is temporary.
        if PLIC_BASE <= addr && addr <= PLIC_BASE + UART_IRQ * 4 {
            if addr == PLIC_BASE + UART_IRQ * 4 {
                return self.priorities[0];
            }
            if addr == PLIC_BASE + VIRTIO_IRQ * 4 {
                return self.priorities[1];
            }
        }
        if addr == PLIC_PENDING {
            return self.pending;
        }
        if PLIC_SENABLE <= addr && addr <= PLIC_SENABLE + 0x100 * self.senables.len() as u64 {
            let index = (addr - PLIC_SENABLE) / 0x100;

            return self.senables[index as usize];
        }
        if PLIC_SPRIORITY <= addr && addr <= PLIC_SPRIORITY + 0x2000 * self.spriorities.len() as u64
        {
            let index = (addr - PLIC_SPRIORITY) / 0x2000;
            return self.spriorities[index as usize];
        }
        if PLIC_SCLAIM <= addr && addr <= PLIC_SCLAIM + 0x2000 * self.sclaims.len() as u64 {
            let index = (addr - PLIC_SCLAIM) / 0x2000;
            return self.sclaims[index as usize];
        }
        0
    }

    /// Write 4 bytes to the PLIC only if the address is valid.
    pub fn write(&mut self, addr: u64, val: u32) {
        // TODO: This `if` statement is temporary.
        if PLIC_BASE <= addr && addr <= PLIC_BASE + UART_IRQ * 4 {
            if addr == PLIC_BASE + UART_IRQ * 4 {
                self.priorities[0] = val;
                return;
            }
            if addr == PLIC_BASE + VIRTIO_IRQ * 4 {
                self.priorities[1] = val;
                return;
            }
        }
        if addr == PLIC_PENDING {
            self.pending = val;
            return;
        }
        if PLIC_SENABLE <= addr && addr <= PLIC_SENABLE + 0x100 * self.senables.len() as u64 {
            let index = (addr - PLIC_SENABLE) / 0x100;

            self.senables[index as usize] = val;
            return;
        }
        if PLIC_SPRIORITY <= addr && addr <= PLIC_SPRIORITY + 0x2000 * self.spriorities.len() as u64
        {
            let index = (addr - PLIC_SPRIORITY) / 0x2000;
            self.spriorities[index as usize] = val;
            return;
        }
        if PLIC_SCLAIM <= addr && addr <= PLIC_SCLAIM + 0x2000 * self.sclaims.len() as u64 {
            let index = (addr - PLIC_SCLAIM) / 0x2000;
            self.sclaims[index as usize] = val;
            return;
        }
    }
}
