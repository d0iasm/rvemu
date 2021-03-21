//! The plic module contains the platform-level interrupt controller (PLIC). The plic connects all
//! external interrupts in the system to all hart contexts in the system, via the external interrupt
//! source in each hart. It's the global interrupt controller in a RISC-V system.

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
use crate::cpu::WORD;
use crate::exception::Exception;

/// The address that interrupt source priority starts.
const PLIC_SOURCE_PRIORITY: u64 = PLIC_BASE;
/// The address that interrupt source priority ends. 1024 4-byte registers exist.
const PLIC_SOURCE_PRIORITY_END: u64 = PLIC_SOURCE_PRIORITY + 0xfff;

/// The address that interrupt pending bits start.
const PLIC_PENDING: u64 = PLIC_BASE + 0x1000;
/// The address that interrupt pending bits end. 32 4-byte (1024 bits) registers exist.
const PLIC_PENDING_END: u64 = PLIC_PENDING + 0x7f;

/// The address that the regsiters to enable interrupts start.
const PLIC_ENABLE: u64 = PLIC_BASE + 0x2000;
/// The address that the regsiters to enable interrupts end. The maximum number of contexts is
/// 15871 but this PLIC supports 2 contexts.
/// base + 0x002000: Enable bits for sources 0-31 on context 0
/// base + 0x002004: Enable bits for sources 32-63 on context 0
/// ...
/// base + 0x00207F: Enable bits for sources 992-1023 on context 0
/// base + 0x002080: Enable bits for sources 0-31 on context 1
/// base + 0x002084: Enable bits for sources 32-63 on context 1
/// ...
/// base + 0x0020FF: Enable bits for sources 992-1023 on context 1
const PLIC_ENABLE_END: u64 = PLIC_ENABLE + 0xff;

/// The address that priority thresholds and claim/complete registers start.
const PLIC_THRESHOLD_AND_CLAIM: u64 = PLIC_BASE + 0x200000;
/// The address that priority thresholds and claim/complete registers end.  The maximum number of
/// contexts is 15871 but this PLIC supports 2 contexts.
/// base + 0x200000: Priority threshold for context 0
/// base + 0x200004: Claim/complete for context 0
/// base + 0x200008: Reserved
/// ...
/// base + 0x200FFC: Reserved
/// base + 0x201000: Priority threshold for context 1
/// base + 0x201004: Claim/complete for context 1
const PLIC_THRESHOLD_AND_CLAIM_END: u64 = PLIC_THRESHOLD_AND_CLAIM + 0x1007;

/// The address of the claim/complete registers for S-mode (context 1).
pub const PLIC_SCLAIM: u64 = PLIC_BASE + 0x201004;

/// The platform-level-interrupt controller (PLIC).
pub struct Plic {
    /// The interrupt priority for each interrupt source. A priority value of 0 is reserved to mean
    /// "never interrupt" and effectively disables the interrupt. Priority 1 is the lowest active
    /// priority, and priority 7 is the highest.
    priority: [u32; 1024],
    /// Interrupt pending bits. If bit 1 is set, a global interrupt 1 is pending. A pending bit in
    /// the PLIC core can be cleared by setting the associated enable bit then performing a claim.
    pending: [u32; 32],
    /// Interrupt Enable Bit of Interrupt Source #0 to #1023 for 2 contexts.
    enable: [u32; 64],
    /// The settings of a interrupt priority threshold of each context. The PLIC will mask all PLIC
    /// interrupts of a priority less than or equal to `threshold`.
    threshold: [u32; 2],
    /// The ID of the highest priority pending interrupt or zero if there is no pending interrupt
    /// for each context.
    claim: [u32; 2],
}

impl Plic {
    /// Create a new PLIC object.
    pub fn new() -> Self {
        Self {
            priority: [0; 1024],
            pending: [0; 32],
            enable: [0; 64],
            threshold: [0; 2],
            claim: [0; 2],
        }
    }

    /// Load `size`-bit data from a register located at `addr` in PLIC.
    pub fn read(&self, addr: u64, size: u8) -> Result<u64, Exception> {
        // TODO: should support byte-base access.
        if size != WORD {
            return Err(Exception::LoadAccessFault);
        }

        match addr {
            PLIC_SOURCE_PRIORITY..=PLIC_SOURCE_PRIORITY_END => {
                let index = (addr - PLIC_SOURCE_PRIORITY).wrapping_div(0x4);
                Ok(self.priority[index as usize] as u64)
            }
            PLIC_PENDING..=PLIC_PENDING_END => {
                let index = (addr - PLIC_PENDING).wrapping_div(0x4);
                Ok(self.pending[index as usize] as u64)
            }
            PLIC_ENABLE..=PLIC_ENABLE_END => {
                let index = (addr - PLIC_ENABLE).wrapping_div(0x4);
                Ok(self.enable[index as usize] as u64)
            }
            PLIC_THRESHOLD_AND_CLAIM..=PLIC_THRESHOLD_AND_CLAIM_END => {
                let context = (addr - PLIC_THRESHOLD_AND_CLAIM).wrapping_div(0x1000);
                let offset = addr - (PLIC_THRESHOLD_AND_CLAIM + 0x1000 * context);
                if offset % 4 == 0 {
                    Ok(self.threshold[context as usize] as u64)
                } else {
                    Ok(self.claim[context as usize] as u64)
                }
            }
            _ => return Err(Exception::LoadAccessFault),
        }
    }

    /// Store `size`-bit data to a register located at `addr` in PLIC.
    pub fn write(&mut self, addr: u64, value: u64, size: u8) -> Result<(), Exception> {
        // TODO: should support byte-base access.
        if size != WORD {
            return Err(Exception::StoreAMOAccessFault);
        }

        match addr {
            PLIC_SOURCE_PRIORITY..=PLIC_SOURCE_PRIORITY_END => {
                let index = (addr - PLIC_SOURCE_PRIORITY).wrapping_div(0x4);
                self.priority[index as usize] = value as u32;
            }
            PLIC_PENDING..=PLIC_PENDING_END => {
                let index = (addr - PLIC_PENDING).wrapping_div(0x4);
                self.pending[index as usize] = value as u32;
            }
            PLIC_ENABLE..=PLIC_ENABLE_END => {
                let index = (addr - PLIC_ENABLE).wrapping_div(0x4);
                self.enable[index as usize] = value as u32;
            }
            PLIC_THRESHOLD_AND_CLAIM..=PLIC_THRESHOLD_AND_CLAIM_END => {
                let context = (addr - PLIC_THRESHOLD_AND_CLAIM).wrapping_div(0x1000);
                let offset = addr - (PLIC_THRESHOLD_AND_CLAIM + 0x1000 * context);
                if offset % 4 == 0 {
                    self.threshold[context as usize] = value as u32;
                } else {
                    self.claim[context as usize] = value as u32;
                }
            }
            _ => return Err(Exception::StoreAMOAccessFault),
        }

        Ok(())
    }
}
