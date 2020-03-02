use crate::csr::*;

pub struct Sepc {
    value: Mxlen,
}

impl CsrBase for Sepc {
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

impl Write for Sepc {}
impl Read for Sepc {}
