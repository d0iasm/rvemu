use crate::csr::*;

/// Encoding of MXL field in `misa`.
#[derive(PartialEq, Eq)]
pub enum Mxl {
    Xlen32 = 1,
    Xlen64 = 2,
    Xlen128 = 3,
}

/// Encoding of extensions field in `misa`.
#[derive(PartialEq, Eq)]
pub enum Extensions {
    /// Atomic extension.
    BitA = 0,
    /// Tentatively reserved for Bit-Manipulation extension.
    BitB = 1,
    /// Compressed extension.
    BitC = 2,
    /// Double-precision floating-point extension.
    BitD = 3,
    /// RV32E base ISA.
    BitE = 4,
    /// Single-precision floating-point extension.
    BitF = 5,
    /// Additional standard extensions present.
    BitG = 6,
    /// Hypervisor extension.
    BitH = 7,
    /// RV32I/64I/128I base ISA.
    BitI = 8,
    /// Tentatively reserved for Dynamically Translated Languages extension.
    BitJ = 9,
    /// Reserved.
    BitK = 10,
    /// Tentatively reserved for Decimal Floating-Point extension.
    BitL = 11,
    /// Integer Multiply/Divide extension.
    BitM = 12,
    /// User-level interrupts supported.
    BitN = 13,
    /// Reserved.
    BitO = 14,
    /// Tentatively reserved for Packed-SIMD extension.
    BitP = 15,
    /// Quad-precision floating-point extension.
    BitQ = 16,
    /// Reserved.
    BitR = 17,
    /// Supervisor mode implemented.
    BitS = 18,
    /// Tentatively reserved for Transactional Memory extension.
    BitT = 19,
    /// User mode implemented.
    BitU = 20,
    /// Tentatively reserved for Vector extension.
    BitV = 21,
    /// Reserved.
    BitW = 22,
    /// Non-standard extensions present.
    BitX = 23,
    /// Reserved.
    BitY = 24,
    /// Reserved.
    BitZ = 25,
}

pub struct Misa {
    value: MXLEN,
}

impl CsrBase for Misa {
    fn new(value: i64) -> Self {
        Self { value }
    }

    fn reset(&mut self) {
        self.value = 1 << Extensions::BitA as i64
                //| 1 << Extensions::BitC as i64
                | 1 << Extensions::BitD as i64
                | 1 << Extensions::BitF as i64
                | 1 << Extensions::BitI as i64
                | 1 << Extensions::BitM as i64
                //| 1 << Extensions::BitN as i64
                | 1 << Extensions::BitS as i64
                | 1 << Extensions::BitU as i64;
    }

    fn set_value(&mut self, value: i64) {
        self.value = value;
    }

    fn get_value(&self) -> i64 {
        self.value
    }
}

impl Write for Misa {}
impl Read for Misa {}

impl Misa {
    pub fn read_mxl(&self) -> Mxl {
        let mxl = self.read_bits(62..);
        match mxl {
            1 => Mxl::Xlen32,
            2 => Mxl::Xlen32,
            3 => Mxl::Xlen32,
            _ => unreachable!("failed to read MXL field in `misa`"),
        }
    }

    pub fn write_mxl(&mut self, value: i64) {
        self.write_bits(62.., value)
    }

    pub fn read_extensions(&self) -> i64 {
        self.read_bits(..26)
    }

    pub fn write_extensions(&mut self, value: i64) {
        self.write_bits(..26, value)
    }
}
