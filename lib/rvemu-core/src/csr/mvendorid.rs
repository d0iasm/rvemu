use crate::csr::*;

pub struct Mvendorid {
    // TODO: The length should be fixed at 32 bits.
    value: Mxlen,
}

impl CsrBase for Mvendorid {
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

impl Read for Mvendorid {}
