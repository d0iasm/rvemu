use crate::csr::*;

pub struct Mimpid {
    value: Mxlen,
}

impl CsrBase for Mimpid {
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

impl Read for Mimpid {}
