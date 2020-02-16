bitflags! {
    #[derive(Default)]
    pub struct Fcsr: u32 {
        /*
         *  31       8 7                   5 4                           0
         * | Reserved | Rounding Mode (frm) |  Accrued Exceptions    (fflags)  |
         *                                  |  NV  |  DZ  |  OF  |  UF  |  NX  |
         *      24              3              1      1      1      1      1
         */
        const RESERVED = 0b11111111_11111111_11111111_00000000;
        const FRM = 0b00000000_00000000_00000000_11100000;
        const NV = 0b00000000_00000000_00000000_00010000;
        const DZ = 0b00000000_00000000_00000000_00001000;
        const OF = 0b00000000_00000000_00000000_00000100;
        const UF = 0b00000000_00000000_00000000_00000010;
        const NX = 0b00000000_00000000_00000000_00000001;
    }
}

#[derive(PartialEq, Eq)]
pub enum RoundingMode {
    Rne = 0b000, // Round to nearest, ties to even.
    Rtz = 0b001, // Round towards zero.
    Rdn = 0b010, // Round down (towards -∞).
    Rup = 0b011, // Round up (towards +∞).
    Rmm = 0b100, // Round to nearest, ties to max maagnitude.
    Dyn = 0b111, // In instruction's rm field, selects dynamic rounding mode; In rounding mode register, invalid.
    Invalid,
}

impl Fcsr {
    pub fn clear(&mut self) {
        self.bits = 0;
    }

    pub fn get_rounding_mode(self) -> RoundingMode {
        let frm = (self & Fcsr::FRM).bits() >> 5;
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
}
