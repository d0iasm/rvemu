use crate::csr::*;

pub struct Mvendorid {
    // TODO: The length should be fixed at 32 bits.
    value: MXLEN,
}

impl CsrBase for Mvendorid {
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

impl Read for Mvendorid {}
