pub mod fcsr;
pub mod marchid;
pub mod medeleg;
pub mod mepc;
pub mod mhartid;
pub mod mimpid;
pub mod misa;
pub mod mstatus;
pub mod mtvec;
pub mod mvendorid;
pub mod sepc;
pub mod uepc;

use std::collections::HashMap;
use std::ops::{Bound, Range, RangeBounds};

use crate::csr::fcsr::Fcsr;
use crate::csr::marchid::Marchid;
use crate::csr::medeleg::Medeleg;
use crate::csr::mepc::Mepc;
use crate::csr::mhartid::Mhartid;
use crate::csr::mimpid::Mimpid;
use crate::csr::misa::Misa;
use crate::csr::mstatus::Mstatus;
use crate::csr::mtvec::Mtvec;
use crate::csr::mvendorid::Mvendorid;
use crate::csr::sepc::Sepc;
use crate::csr::uepc::Uepc;
use crate::exception::Exception;

pub type CsrAddress = u32;

//////////////////////////////
// User-level CSR addresses //
//////////////////////////////
// User trap handling.
/// User exception program counter.
pub const UEPC: CsrAddress = 0x041;
/// User trap cause.
pub const UCAUSE: CsrAddress = 0x042;

// User floating-point CSRs.
/// Flating-point accrued exceptions.
pub const FFLAGS: CsrAddress = 0x001;
/// Floating-point dynamic rounding mode.
pub const FRB: CsrAddress = 0x002;
/// Floating-point control and status register (frm + fflags).
pub const FCSR: CsrAddress = 0x003;

/////////////////////////////////////
// Supervisor-level CSR addresses //
////////////////////////////////////
// Supervisor trap handling.
/// Supervisor exception program counter.
pub const SEPC: CsrAddress = 0x141;
/// Supervisor trap cause.
pub const SCAUSE: CsrAddress = 0x142;

// Supervisor protection and translation.
/// Supervisor address translation and protection.
pub const SATP: CsrAddress = 0x180;

/////////////////////////////////
// Machine-level CSR addresses //
/////////////////////////////////
// Machine information registers.
/// Vendor ID.
pub const MVENDORID: CsrAddress = 0xf11;
/// Architecture ID.
pub const MARCHID: CsrAddress = 0xf12;
/// Implementation ID.
pub const MIMPID: CsrAddress = 0xf13;
/// Hardware thread ID.
pub const MHARTID: CsrAddress = 0xf14;

// Machine trap setup.
/// Machine status register.
pub const MSTATUS: CsrAddress = 0x300;
/// ISA and extensions.
pub const MISA: CsrAddress = 0x301;
/// Machine exception delefation register.
pub const MEDELEG: CsrAddress = 0x302;
/// Machine interrupt delefation register.
pub const MIDELEG: CsrAddress = 0x303;
/// Machine interrupt-enable register.
pub const MIE: CsrAddress = 0x304;
/// Machine trap-handler base address.
pub const MTVEC: CsrAddress = 0x305;
/// Machine counter enable.
pub const MCOUNTEREN: CsrAddress = 0x306;

// Machine trap handling.
/// Scratch register for machine trap handlers.
pub const MSCRATCH: CsrAddress = 0x340;
/// Machine exception program counter.
pub const MEPC: CsrAddress = 0x341;
/// Machine trap cause.
pub const MCAUSE: CsrAddress = 0x342;
/// Machine bad address or instruction.
pub const MTVAL: CsrAddress = 0x343;
/// Machine interrupt pending.
pub const MIP: CsrAddress = 0x344;

pub type MXLEN = i64;

pub struct State {
    csrs: HashMap<CsrAddress, Csr>,
}

pub enum Csr {
    // User-level CSRs.
    Uepc(Uepc),
    Fcsr(Fcsr),
    // Supervisor-level CSRs.
    Sepc(Sepc),
    // Machine-level CSRs.
    Mvendorid(Mvendorid),
    Marchid(Marchid),
    Mimpid(Mimpid),
    Mhartid(Mhartid),
    Mstatus(Mstatus),
    Misa(Misa),
    Medeleg(Medeleg),
    Mtvec(Mtvec),
    Mepc(Mepc),
}

impl State {
    pub fn new() -> Self {
        let mut csrs = HashMap::new();

        // User-level CSRs.
        csrs.insert(UEPC, Csr::Uepc(Uepc::new(0)));

        csrs.insert(FCSR, Csr::Fcsr(Fcsr::new(0)));

        // Supervisor-level CSRs.
        csrs.insert(SEPC, Csr::Sepc(Sepc::new(0)));

        // Machine-level CSRs.
        csrs.insert(MVENDORID, Csr::Mvendorid(Mvendorid::new(0)));
        csrs.insert(MARCHID, Csr::Marchid(Marchid::new(0)));
        csrs.insert(MIMPID, Csr::Mimpid(Mimpid::new(0)));
        csrs.insert(MHARTID, Csr::Mhartid(Mhartid::new(0)));

        csrs.insert(MSTATUS, Csr::Mstatus(Mstatus::new(0)));
        csrs.insert(MISA, Csr::Misa(Misa::new(0)));
        csrs.insert(MEDELEG, Csr::Medeleg(Medeleg::new(0)));
        csrs.insert(MTVEC, Csr::Mtvec(Mtvec::new(0)));

        csrs.insert(MEPC, Csr::Mepc(Mepc::new(0)));

        /*
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
        */

        Self { csrs }
    }

    pub fn get(&mut self, csr_address: CsrAddress) -> Result<&mut Csr, Exception> {
        if let Some(csr) = self.csrs.get_mut(&csr_address) {
            Ok(csr)
        } else {
            Err(Exception::IllegalInstruction(String::from(
                "failed to get a csr",
            )))
        }
    }

    pub fn read(&self, csr_address: u32) -> Result<MXLEN, Exception> {
        if let Some(csr) = self.csrs.get(&csr_address) {
            match csr {
                Csr::Uepc(uepc) => Ok(uepc.read_value()),
                Csr::Fcsr(fcsr) => Ok(fcsr.read_value()),
                Csr::Sepc(sepc) => Ok(sepc.read_value()),
                Csr::Mvendorid(mvendorid) => Ok(mvendorid.read_value()),
                Csr::Marchid(marchid) => Ok(marchid.read_value()),
                Csr::Mimpid(mimpid) => Ok(mimpid.read_value()),
                Csr::Mhartid(mhartid) => Ok(mhartid.read_value()),
                Csr::Mstatus(mstatus) => Ok(mstatus.read_value()),
                Csr::Misa(misa) => Ok(misa.read_value()),
                Csr::Medeleg(medeleg) => Ok(medeleg.read_value()),
                Csr::Mtvec(mtvec) => Ok(mtvec.read_value()),
                Csr::Mepc(mepc) => Ok(mepc.read_value()),
            }
        } else {
            Err(Exception::IllegalInstruction(String::from(
                "failed to read a value from a csr",
            )))
        }
    }

    pub fn write(&mut self, csr_address: u32, value: MXLEN) -> Result<(), Exception> {
        if let Some(csr) = self.csrs.get_mut(&csr_address) {
            match csr {
                Csr::Uepc(uepc) => uepc.write_value(value),
                Csr::Fcsr(fcsr) => fcsr.write_value(value),
                Csr::Sepc(sepc) => sepc.write_value(value),
                Csr::Mvendorid(_) => {
                    return Err(Exception::IllegalInstruction(String::from(
                        "mvendorid is a read-only csr",
                    )))
                }
                Csr::Marchid(_) => {
                    return Err(Exception::IllegalInstruction(String::from(
                        "marchid is a read-only csr",
                    )))
                }
                Csr::Mimpid(_) => {
                    return Err(Exception::IllegalInstruction(String::from(
                        "mimpid is a read-only csr",
                    )))
                }
                Csr::Mhartid(_) => {
                    return Err(Exception::IllegalInstruction(String::from(
                        "mhartid is a read-only csr",
                    )))
                }
                Csr::Mstatus(mstatus) => mstatus.write_value(value),
                Csr::Misa(misa) => misa.write_value(value),
                Csr::Medeleg(medeleg) => medeleg.write_value(value),
                Csr::Mtvec(mtvec) => mtvec.write_value(value),
                Csr::Mepc(mepc) => mepc.write_value(value),
            }
            Ok(())
        } else {
            Err(Exception::IllegalInstruction(String::from(
                "failed to write a value to a csr",
            )))
        }
    }

    pub fn reset(&mut self) {
        for csr in self.csrs.values_mut() {
            match csr {
                Csr::Uepc(uepc) => uepc.reset(),
                Csr::Fcsr(fcsr) => fcsr.reset(),
                Csr::Sepc(sepc) => sepc.reset(),
                Csr::Mvendorid(mvendorid) => mvendorid.reset(),
                Csr::Marchid(marchid) => marchid.reset(),
                Csr::Mimpid(mimpid) => mimpid.reset(),
                Csr::Mhartid(mhartid) => mhartid.reset(),
                Csr::Mstatus(mstatus) => mstatus.reset(),
                Csr::Misa(misa) => misa.reset(),
                Csr::Medeleg(medeleg) => medeleg.reset(),
                Csr::Mtvec(mtvec) => mtvec.reset(),
                Csr::Mepc(mepc) => mepc.reset(),
            }
        }
    }
}

pub trait CsrBase {
    const BIT_LENGTH: usize = ::core::mem::size_of::<i64>() as usize * 8;

    fn new(value: MXLEN) -> Self;
    fn reset(&mut self);
    fn write_value(&mut self, value: MXLEN);
    fn read_value(&self) -> MXLEN;
}

pub trait Write: CsrBase {
    fn write_bit(&mut self, bit: usize, value: bool) {
        if bit >= Self::BIT_LENGTH {
            // TODO: raise exception?
        }

        if value {
            self.write_value(self.read_value() | 1 << bit);
        } else {
            self.write_value(self.read_value() & !(1 << bit));
        }
    }

    fn write_bits<T: RangeBounds<usize>>(&mut self, range: T, value: MXLEN) {
        let range = to_range(&range, Self::BIT_LENGTH);

        if (range.start >= Self::BIT_LENGTH)
            | (range.end > Self::BIT_LENGTH)
            | (range.start >= range.end)
        {
            // TODO: ranse exception?
        }

        let bitmask = (!0 << range.end) | !(!0 << range.start);
        // Set bits.
        self.write_value((self.read_value() & bitmask) | (value << range.start))
    }
}

pub trait Read: CsrBase {
    fn read_bit(&self, bit: usize) -> bool {
        if bit >= Self::BIT_LENGTH {
            // TODO: raise exception?
        }
        (self.read_value() & (1 << bit)) != 0
    }

    fn read_bits<T: RangeBounds<usize>>(&self, range: T) -> i64 {
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
        (self.read_value() & !bitmask) >> range.start
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
