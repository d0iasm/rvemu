use crate::csr::*;

pub struct Pmpcfg0 {
    value: Mxlen,
}

impl CsrBase for Pmpcfg0 {
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

impl Write for Pmpcfg0 {}
impl Read for Pmpcfg0 {}
