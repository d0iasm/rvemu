use crate::cpu::Mode;
use crate::csr::*;

pub struct Mstatus {
    value: MXLEN,
}

impl CsrBase for Mstatus {
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

impl Read for Mstatus {}
impl Write for Mstatus {}

impl Mstatus {
    /// Read a privious privilege mode for machine mode.
    pub fn read_mpp(&self) -> Mode {
        let mpp = self.read_bits(11..13);
        match mpp {
            0b00 => Mode::User,
            0b01 => Mode::Supervisor,
            0b11 => Mode::Machine,
            _ => Mode::Debug,
        }
    }

    /// Write a privious privilege mode for machine mode.
    pub fn write_mpp(&mut self, mode: Mode) {
        match mode {
            Mode::User => self.write_bits(11..13, 0b00),
            Mode::Supervisor => self.write_bits(11..13, 0b01),
            Mode::Machine => self.write_bits(11..13, 0b11),
            _ => self.write_bits(11..13, 0b00),
        }
    }

    /// Read a privious privilege mode for supervisor mode. It can only holds machine
    /// or supervisor mode.
    pub fn read_spp(&self) -> Mode {
        let spp = self.read_bit(8);
        match spp {
            false => Mode::User,
            true => Mode::Supervisor,
        }
    }

    /// Write a privious privilege mode for supervisor mode. It can only holds machine or
    /// supervisor mode.
    pub fn write_spp(&mut self, mode: Mode) {
        match mode {
            Mode::User => self.write_bit(8, false),
            Mode::Supervisor => self.write_bit(8, true),
            _ => self.write_bit(8, false),
        }
    }

    /// Read a privious interrupt-enable bit for machine mode.
    pub fn read_mpie(&self) -> bool {
        self.read_bit(7)
    }

    /// Write a privious interrupt-enable bit for machine mode.
    pub fn write_mpie(&mut self, value: bool) {
        self.write_bit(7, value)
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

    /// Read a global interrupt-enable bit for machine mode.
    pub fn read_mie(&self) -> bool {
        self.read_bit(3)
    }

    /// Write a global interrupt-enable bit for machine mode.
    pub fn write_mie(&mut self, value: bool) {
        self.write_bit(3, value)
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
