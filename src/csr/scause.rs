use crate::csr::*;

pub struct Scause {
    value: Mxlen,
}

impl CsrBase for Scause {
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

impl Write for Scause {}
impl Read for Scause {}

impl Scause {
    pub fn read_interrupt(&self) -> bool {
        self.read_bit(MXLEN - 1)
    }

    pub fn write_interrupt(&mut self, value: bool) {
        self.write_bit(MXLEN - 1, value)
    }

    pub fn read_exception_code(&self) -> Mxlen {
        self.read_bits(..MXLEN - 1)
    }

    pub fn write_exception_code(&mut self, value: Mxlen) {
        self.write_bits(..MXLEN - 1, value)
    }
}
