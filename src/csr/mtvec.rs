use crate::csr::*;

pub enum Mode {
    /// All exceptions set pc to BASE.
    Direct = 0,
    /// Asynchronous interrupts set pc to BASE + 4 * cause.
    Vectored = 1,
    /// Reserved.
    Reserved,
}

pub struct Mtvec {
    value: MXLEN,
}

impl CsrBase for Mtvec {
    fn new(value: MXLEN) -> Self {
        Self { value }
    }

    fn reset(&mut self) {
        self.value = 0;
    }

    fn write_value(&mut self, value: MXLEN) {
        self.value = value;
    }

    fn read_value(&self) -> MXLEN {
        self.value
    }
}

impl Write for Mtvec {}
impl Read for Mtvec {}

impl Mtvec {
    pub fn read_base(&self) -> MXLEN {
        self.read_bits(2..)
    }

    pub fn write_base(&mut self, base: MXLEN) {
        // The BASE field must always be aaligned on a 4-byte boundary.
        self.write_bits(2.., base)
    }

    pub fn read_mode(&self) -> Mode {
        match self.read_bits(..2) {
            0 => Mode::Direct,
            1 => Mode::Vectored,
            _ => Mode::Reserved,
        }
    }

    pub fn write_mode(&mut self, mode: Mode) {
        match mode {
            Mode::Direct => self.write_bits(..2, 0),
            Mode::Vectored => self.write_bits(..2, 1),
            _ => {}
        }
    }
}
