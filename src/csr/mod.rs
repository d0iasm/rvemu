pub mod fcsr;

use std::collections::HashMap;
use std::ops::{Bound, Range, RangeBounds};

use crate::exception::Exception;

pub type CsrAddress = u32;

//////////////////////////////
// User-level CSR addresses //
//////////////////////////////
// User trap handling.
pub const UEPC: CsrAddress = 0x041; // User exception program counter.
pub const UCAUSE: CsrAddress = 0x042; // User trap cause.

// User floating-point CSRs.
pub const FFLAGS: CsrAddress = 0x001; // Flating-point accrued exceptions.
pub const FRB: CsrAddress = 0x002; // Floating-point dynamic rounding mode.
pub const FCSR: CsrAddress = 0x003; // Floating-point control and status register (frm + fflags).

////////////////////////////////////
// Supervisor-level CSR addresses //
////////////////////////////////////
// Supervisor trap handling.
pub const SEPC: CsrAddress = 0x141; // Supervisor exception program counter.
pub const SCAUSE: CsrAddress = 0x142; // Supervisor trap cause.

/////////////////////////////////
// Machine-level CSR addresses //
/////////////////////////////////
// Machine information registers.
pub const MHARTID: CsrAddress = 0xf14; // Hardware thread ID.

// Machine trap setup.
pub const MSTATUS: CsrAddress = 0x300; // Machine status register.
pub const MISA: CsrAddress = 0x301; // ISA and extensions.
pub const MEDELEG: CsrAddress = 0x302; // Machine exception delefation register.
pub const MIDELEG: CsrAddress = 0x303; // Machine interrupt delefation register.
pub const MIE: CsrAddress = 0x304; // Machine interrupt-enable register.
pub const MTVEC: CsrAddress = 0x305; // Machine trap-handler base address.
pub const MCOUNTEREN: CsrAddress = 0x306; // Machine counter enable.

// Machine trap handling.
pub const MSCRATCH: CsrAddress = 0x340; // Scratch register for machine trap handlers.
pub const MEPC: CsrAddress = 0x341; // Machine exception program counter.
pub const MCAUSE: CsrAddress = 0x342; // Machine trap cause.
pub const MTVAL: CsrAddress = 0x343; // Machine bad address or instruction.
pub const MIP: CsrAddress = 0x344; // Machine interrupt pending.

pub struct State {
    csrs: HashMap<CsrAddress, Csr>,
}

impl State {
    pub fn new() -> Self {
        let mut csrs = HashMap::new();

        // csr[11:10]: Whether the register is read/write (00, 01, or 10) or read-only (11).
        // csr[9:8]: The lowest privilege level that can access the CSR. User (00), supervisor
        // (01), hypervisor (10), and machine (11).
        csrs.insert(UEPC, Csr::RW(ReadWrite::new(0)));
        csrs.insert(UCAUSE, Csr::RW(ReadWrite::new(0)));
        csrs.insert(FFLAGS, Csr::RW(ReadWrite::new(0)));
        csrs.insert(FRB, Csr::RW(ReadWrite::new(0)));
        csrs.insert(FCSR, Csr::RW(ReadWrite::new(0)));

        csrs.insert(SEPC, Csr::RW(ReadWrite::new(0)));
        csrs.insert(SCAUSE, Csr::RW(ReadWrite::new(0)));

        csrs.insert(MHARTID, Csr::RO(ReadOnly::new(0)));
        csrs.insert(MSTATUS, Csr::RW(ReadWrite::new(0)));
        csrs.insert(MISA, Csr::RW(ReadWrite::new(0)));
        csrs.insert(MEDELEG, Csr::RW(ReadWrite::new(0)));
        csrs.insert(MIDELEG, Csr::RW(ReadWrite::new(0)));
        csrs.insert(MIE, Csr::RW(ReadWrite::new(0)));
        csrs.insert(MTVEC, Csr::RW(ReadWrite::new(0)));
        csrs.insert(MCOUNTEREN, Csr::RW(ReadWrite::new(0)));
        csrs.insert(MSCRATCH, Csr::RW(ReadWrite::new(0)));
        csrs.insert(MEPC, Csr::RW(ReadWrite::new(0)));
        csrs.insert(MCAUSE, Csr::RW(ReadWrite::new(0)));
        csrs.insert(MTVAL, Csr::RW(ReadWrite::new(0)));
        csrs.insert(MIP, Csr::RW(ReadWrite::new(0)));

        Self { csrs }
    }

    pub fn get(&self, csr_address: CsrAddress) -> Result<&Csr, Exception> {
        if let Some(csr) = self.csrs.get(&csr_address) {
            Ok(csr)
        } else {
            Err(Exception::IllegalInstruction(String::from(
                "failed to get a csr.",
            )))
        }
    }

    pub fn read(&self, csr_address: u32) -> Result<i64, Exception> {
        if let Some(csr) = self.csrs.get(&csr_address) {
            match csr {
                Csr::RW(c) => Ok(c.read_bits(..)),
                Csr::RO(c) => Ok(c.read_bits(..)),
            }
        } else {
            Err(Exception::IllegalInstruction(String::from(
                "failed to read a csr.",
            )))
        }
    }

    pub fn write(&mut self, csr_address: u32, value: i64) -> Result<(), Exception> {
        if let Some(csr) = self.csrs.get_mut(&csr_address) {
            match csr {
                Csr::RW(c) => Ok(c.write_bits(.., value)),
                Csr::RO(_c) => Err(Exception::IllegalInstruction(String::from(
                    "failed to write a read-only csr.",
                ))),
            }
        } else {
            Err(Exception::IllegalInstruction(String::from(
                "failed to write a csr.",
            )))
        }
    }

    pub fn clear(&mut self) {
        for csr in self.csrs.values_mut() {
            // TODO: Use ReadOnly::new(0) depends on a csr.
            *csr = Csr::RW(ReadWrite::new(0));
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Csr {
    RW(ReadWrite),
    RO(ReadOnly),
}

#[derive(Default, Copy, Clone, Debug)]
pub struct ReadWrite {
    value: i64,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct ReadOnly {
    value: i64,
}

impl ReadWrite {
    const BIT_LENGTH: usize = ::core::mem::size_of::<i64>() as usize * 8;

    pub fn new(value: i64) -> Self {
        Self { value }
    }

    pub fn clear(&mut self) {
        self.value = 0;
    }

    pub fn write_bit(&mut self, bit: usize, value: bool) {
        if bit >= Self::BIT_LENGTH {
            // TODO: raise exception?
        }

        if value {
            self.value |= 1 << bit;
        } else {
            self.value &= !(1 << bit);
        }
    }

    pub fn write_bits<T: RangeBounds<usize>>(&mut self, range: T, value: i64) {
        let range = to_range(&range, Self::BIT_LENGTH);

        if (range.start >= Self::BIT_LENGTH)
            | (range.end > Self::BIT_LENGTH)
            | (range.start >= range.end)
        {
            // TODO: ranse exception?
        }

        let bitmask = (!0 << range.end) | !(!0 << range.start);
        // Set bits.
        self.value = (self.value & bitmask) | (value << range.start);
    }

    pub fn read_bit(&self, bit: usize) -> bool {
        if bit >= Self::BIT_LENGTH {
            // TODO: raise exception?
        }
        (self.value & (1 << bit)) != 0
    }

    pub fn read_bits<T: RangeBounds<usize>>(&self, range: T) -> i64 {
        let range = to_range(&range, Self::BIT_LENGTH);

        if (range.start >= Self::BIT_LENGTH)
            | (range.end > Self::BIT_LENGTH)
            | (range.start >= range.end)
        {
            // TODO: ranse exception?
        }

        // Bitmask for high bits.
        let bitmask = !0 << range.end;

        // Shift away low bits.
        (self.value & !bitmask) >> range.start
    }
}

impl ReadOnly {
    const BIT_LENGTH: usize = ::core::mem::size_of::<i64>() as usize * 8;

    pub fn new(value: i64) -> Self {
        Self { value }
    }

    pub fn clear(&mut self) {
        self.value = 0;
    }

    pub fn read_bit(&self, bit: usize) -> bool {
        if bit >= Self::BIT_LENGTH {
            // TODO: raise exception?
        }
        (self.value & (1 << bit)) != 0
    }

    pub fn read_bits<T: RangeBounds<usize>>(&self, range: T) -> i64 {
        let range = to_range(&range, Self::BIT_LENGTH);

        if (range.start >= Self::BIT_LENGTH)
            | (range.end > Self::BIT_LENGTH)
            | (range.start >= range.end)
        {
            // TODO: ranse exception?
        }

        // Bitmask for high bits.
        let bitmask = !0 << range.end;

        // Shift away low bits.
        (self.value & !bitmask) >> range.start
    }
}

fn to_range<T: RangeBounds<usize>>(generic_range: &T, bit_length: usize) -> Range<usize> {
    let start = match generic_range.start_bound() {
        Bound::Excluded(&value) => value + 1,
        Bound::Included(&value) => value,
        Bound::Unbounded => 0,
    };
    let end = match generic_range.end_bound() {
        Bound::Excluded(&value) => value,
        Bound::Included(&value) => value + 1,
        Bound::Unbounded => bit_length,
    };

    start..end
}
