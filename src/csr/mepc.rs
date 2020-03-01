use crate::csr::*;

pub struct Mepc {
    value: MXLEN,
}

impl CsrBase for Mepc {
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

impl Write for Mepc {}
impl Read for Mepc {}
