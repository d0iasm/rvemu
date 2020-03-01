use crate::csr::*;

pub struct Uepc {
    value: MXLEN,
}

impl CsrBase for Uepc {
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

impl Write for Uepc {}
impl Read for Uepc {}
