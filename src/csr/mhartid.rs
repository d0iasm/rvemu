use crate::csr::*;

pub struct Mhartid {
    value: Mxlen,
}

impl CsrBase for Mhartid {
    // HartIDs might not necessarily be numbered contiguously in a multiprocessor system, but at
    // least one hart must have a hart ID of zero. Hart IDs must be unique.
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

impl Read for Mhartid {}
