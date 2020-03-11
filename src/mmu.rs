//! The mmu module contains the functions to translate a virtual memory to a physical memory.

use crate::csr::satp;
use crate::exception::Exception;

/// Translate a virtual address to a physical address for the paged virtual-memory system.
pub fn translate(address: usize, mode: satp::Mode) -> Result<usize, Exception> {
    match mode {
        satp::Mode::Bare => Ok(address),
        satp::Mode::Sv39 => Ok(address),
        satp::Mode::Sv48 => Ok(address),
        _ => Err(Exception::InstructionPageFault),
    }
}
