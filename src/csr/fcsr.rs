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

impl Fcsr {
    pub fn clear(&mut self) {
        self.bits = 0;
    }
}
