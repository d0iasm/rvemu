use crate::csr::*;

pub struct Mscratch {
    value: Mxlen,
}

impl CsrBase for Mscratch {
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

impl Write for Mscratch {}
impl Read for Mscratch {}
