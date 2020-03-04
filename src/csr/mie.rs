use crate::csr::*;

pub struct Mie {
    value: Mxlen,
}

impl CsrBase for Mie {
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

impl Write for Mie {}
impl Read for Mie {}
