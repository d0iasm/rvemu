use crate::csr::*;

pub struct Mip {
    value: Mxlen,
}

impl CsrBase for Mip {
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

impl Write for Mip {}
impl Read for Mip {}
