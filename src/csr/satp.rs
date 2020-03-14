//! Supervisor address translation and protection.

use crate::csr::*;

#[derive(Debug)]
pub enum Mode {
    /// No translation or protection.
    Bare = 0,
    /// Page-based 39-bit virtual addressing.
    Sv39 = 8,
    /// Page-based 48-bit virtual addressing.
    Sv48 = 9,
    /// Reserved.
    /// (Sv57) Reserved for page-based 57-bit virtual addressing.
    /// (Sv64) Reserved for page-based 64-bit virtual addressing.
    Reserved,
}

pub struct Satp {
    value: Mxlen,
}

impl CsrBase for Satp {
    fn new(value: Mxlen) -> Self {
        Self { value }
    }

    fn reset(&mut self) {
        self.value = 0;
    }

    fn write_value(&mut self, value: Mxlen) {
        self.value = value;
    }

    fn read_value(&self) -> Mxlen {
        self.value
    }
}

impl Write for Satp {}
impl Read for Satp {}

impl Satp {
    /// Read the MODE field, which selects the current address-translation scheme.
    pub fn read_mode(&self) -> Mode {
        match self.read_bits(60..) {
            0 => Mode::Bare,
            8 => Mode::Sv39,
            9 => Mode::Sv48,
            _ => Mode::Reserved,
        }
    }

    /// Write the MODE field, which selects the current address-translation scheme.
    pub fn write_mode(&mut self, mode: Mode) {
        match mode {
            Mode::Bare => self.write_bits(60.., 0),
            Mode::Sv39 => self.write_bits(60.., 8),
            Mode::Sv48 => self.write_bits(60.., 9),
            _ => {}
        }
    }

    ///  Read the address space identifier (ASID), which facilitates
    ///  address-translation fences on a per-address-space basis.
    pub fn read_asid(&self) -> Mxlen {
        self.read_bits(44..60)
    }

    ///  Write the address space identifier (ASID), which facilitates
    ///  address-translation fences on a per-address-space basis.
    pub fn write_asid(&mut self, value: Mxlen) {
        self.write_bits(44..60, value);
    }

    /// Read the physical page number (PPN) of the root page table, i.e., its
    /// supervisor physical address divided by 4 KiB.
    pub fn read_ppn(&self) -> Mxlen {
        self.read_bits(..44)
    }

    /// Write the physical page number (PPN) of the root page table, i.e., its
    /// supervisor physical address divided by 4 KiB.
    pub fn write_ppn(&mut self, value: Mxlen) {
        self.write_bits(..44, value);
    }
}
