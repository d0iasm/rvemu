use std::convert::From;
use std::unreachable;

use crate::csr::*;

#[derive(PartialEq, Eq)]
pub enum RoundingMode {
    /// Round to nearest, ties to even.
    Rne = 0b000,
    /// Round towards zero.
    Rtz = 0b001,
    /// Round down (towards -∞).
    Rdn = 0b010,
    /// Round up (towards +∞).
    Rup = 0b011,
    /// Round to nearest, ties to max maagnitude.
    Rmm = 0b100,
    /// In instruction's rm field, selects dynamic rounding mode; In rounding mode register, invalid.
    Dyn = 0b111,
    Invalid,
}

pub struct Fcsr {
    csr: ReadWrite,
}

impl From<Csr> for Fcsr {
    fn from(csr: Csr) -> Self {
        match csr {
            Csr::RW(csr) => Self { csr },
            _ => unreachable!(),
        }
    }
}

impl Fcsr {
    /*
     *  31       8 7                   5 4                           0
     * | Reserved | Rounding Mode (frm) | Accrued Exceptions: fflags |
     *                                  |   NV | DZ | OF | UF | NX   |
     *      24              3               1    1    1    1    1
     */
    pub fn clear(&mut self) {
        self.csr.clear();
    }

    pub fn read_frm(&self) -> RoundingMode {
        let frm = self.csr.read_bits(5..8);
        match frm {
            0b000 => RoundingMode::Rne,
            0b001 => RoundingMode::Rtz,
            0b010 => RoundingMode::Rdn,
            0b011 => RoundingMode::Rup,
            0b100 => RoundingMode::Rmm,
            0b111 => RoundingMode::Dyn,
            _ => RoundingMode::Invalid,
        }
    }

    pub fn read_nv(&self) -> bool {
        self.csr.read_bit(4)
    }

    pub fn write_nv(&mut self, value: bool) {
        self.csr.write_bit(4, value)
    }

    pub fn read_dz(&self) -> bool {
        self.csr.read_bit(3)
    }

    pub fn write_dz(&mut self, value: bool) {
        self.csr.write_bit(3, value)
    }

    pub fn read_of(&self) -> bool {
        self.csr.read_bit(2)
    }

    pub fn write_of(&mut self, value: bool) {
        self.csr.write_bit(2, value)
    }

    pub fn read_uf(&self) -> bool {
        self.csr.read_bit(1)
    }

    pub fn write_uf(&mut self, value: bool) {
        self.csr.write_bit(1, value)
    }

    pub fn read_nx(&self) -> bool {
        self.csr.read_bit(0)
    }

    pub fn write_nx(&mut self, value: bool) {
        self.csr.write_bit(0, value)
    }
}
