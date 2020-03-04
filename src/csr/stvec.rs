use crate::csr::*;

pub enum Mode {
    /// All exceptions set pc to BASE.
    Direct = 0,
    /// Asynchronous interrupts set pc to BASE + 4 * cause.
    Vectored = 1,
    /// Reserved.
    Reserved,
}

pub struct Stvec {
    value: Mxlen,
}

impl CsrBase for Stvec {
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

impl Write for Stvec {}
impl Read for Stvec {}

impl Stvec {
    pub fn read_base(&self) -> Mxlen {
        self.read_bits(2..)
    }

    pub fn write_base(&mut self, base: Mxlen) {
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
