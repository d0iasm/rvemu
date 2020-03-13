use crate::csr::*;

pub struct Sstatus {
    value: Mxlen,
}

impl CsrBase for Sstatus {
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

impl Read for Sstatus {}
impl Write for Sstatus {}

impl Sstatus {
    /// Read a privious privilege mode for supervisor mode.
    /// or supervisor mode.
    pub fn read_spp(&self) -> bool {
        self.read_bit(8)
    }

    /// Write a privious privilege mode for supervisor mode.
    pub fn write_spp(&mut self, mode: bool) {
        self.write_bit(8, mode)
    }

    /// Read a privious interrupt-enable bit for supervisor mode.
    pub fn read_spie(&self) -> bool {
        self.read_bit(5)
    }

    /// Write a privious interrupt-enable bit for supervisor mode.
    pub fn write_spie(&mut self, value: bool) {
        self.write_bit(5, value)
    }

    /// Read a privious interrupt-enable bit for user mode.
    pub fn read_upie(&self) -> bool {
        self.read_bit(4)
    }

    /// Write a privious interrupt-enable bit for user mode.
    pub fn write_upie(&mut self, value: bool) {
        self.write_bit(4, value)
    }

    /// Read a global interrupt-enable bit for supervisor mode.
    pub fn read_sie(&self) -> bool {
        self.read_bit(1)
    }

    /// Write a global interrupt-enable bit for supervisor mode.
    pub fn write_sie(&mut self, value: bool) {
        self.write_bit(1, value)
    }

    /// Read a global interrupt-enable bit for user mode.
    pub fn read_uie(&self) -> bool {
        self.read_bit(0)
    }

    /// Write a global interrupt-enable bit for user mode.
    pub fn write_uie(&mut self, value: bool) {
        self.write_bit(0, value)
    }
}
