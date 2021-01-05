//! The csr module contains all the control and status registers.

use std::fmt;
use std::ops::{Bound, Range, RangeBounds};

pub type CsrAddress = u16;

pub const MXLEN: usize = 64;
/// The number of CSRs. The field is 12 bits so the maximum kind of CSRs is 4096 (2**12).
pub const CSR_SIZE: usize = 4096;

//////////////////////////////
// User-level CSR addresses //
//////////////////////////////
// User trap setup.
/// User status register.
pub const USTATUS: CsrAddress = 0x000;
/// User trap handler base address.
pub const UTVEC: CsrAddress = 0x005;

// User trap handling.
/// User exception program counter.
pub const UEPC: CsrAddress = 0x041;
/// User trap cause.
pub const UCAUSE: CsrAddress = 0x042;
/// User bad address or instruction.
pub const UTVAL: CsrAddress = 0x043;

// User floating-point CSRs.
/// Flating-point accrued exceptions.
pub const FFLAGS: CsrAddress = 0x001;
/// Floating-point dynamic rounding mode.
pub const FRB: CsrAddress = 0x002;
/// Floating-point control and status register (frm + fflags).
pub const FCSR: CsrAddress = 0x003;

// User Counter/Timers.
/// Timer for RDTIME instruction.
pub const TIME: CsrAddress = 0xc01;

/////////////////////////////////////
// Supervisor-level CSR addresses //
////////////////////////////////////
// Supervisor trap setup.
/// Supervisor status register.
pub const SSTATUS: CsrAddress = 0x100;
/// Supervisor exception delegation register.
pub const SEDELEG: CsrAddress = 0x102;
/// Supervisor interrupt delegation register.
pub const SIDELEG: CsrAddress = 0x103;
/// Supervisor interrupt-enable register.
pub const SIE: CsrAddress = 0x104;
/// Supervisor trap handler base address.
pub const STVEC: CsrAddress = 0x105;

// Supervisor trap handling.
/// Scratch register for supervisor trap handlers.
pub const SSCRATCH: CsrAddress = 0x140;
/// Supervisor exception program counter.
pub const SEPC: CsrAddress = 0x141;
/// Supervisor trap cause.
pub const SCAUSE: CsrAddress = 0x142;
/// Supervisor bad address or instruction.
pub const STVAL: CsrAddress = 0x143;
/// Supervisor interrupt pending.
pub const SIP: CsrAddress = 0x144;

// Supervisor protection and translation.
/// Supervisor address translation and protection.
pub const SATP: CsrAddress = 0x180;

// SSTATUS fields.
//pub const SSTATUS_UIE: u64 = 0x00000001;
pub const SSTATUS_SIE: u64 = 0x00000002;
//pub const SSTATUS_UPIE: u64 = 0x00000010;
pub const SSTATUS_SPIE: u64 = 0x00000020;
pub const SSTATUS_SPP: u64 = 0x00000100;
pub const SSTATUS_VS: u64 = 0x00000600;
pub const SSTATUS_FS: u64 = 0x00006000;
pub const SSTATUS_XS: u64 = 0x00018000;
pub const SSTATUS_SUM: u64 = 0x00040000;
pub const SSTATUS_MXR: u64 = 0x00080000;
pub const SSTATUS_UXL: u64 = 0x0000000300000000;

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

// Machine memory protection.
/// Physical memory protection configuration.
pub const PMPCFG0: CsrAddress = 0x3a0;
/// Physical memory protection address register.
pub const PMPADDR0: CsrAddress = 0x3b0;

// MIP fields.
/// Supervisor software interrupt.
pub const SSIP_BIT: u64 = 1 << 1;
/// Machine software interrupt.
pub const MSIP_BIT: u64 = 1 << 3;
/// Supervisor timer interrupt.
pub const STIP_BIT: u64 = 1 << 5;
/// Machine timer interrupt.
pub const MTIP_BIT: u64 = 1 << 7;
/// Supervisor external interrupt.
pub const SEIP_BIT: u64 = 1 << 9;
/// Machine external interrupt.
pub const MEIP_BIT: u64 = 1 << 11;

/// The state to contains all the CSRs.
pub struct State {
    csrs: [u64; CSR_SIZE],
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "{}\n{}\n{}",
                format!(
                    "mstatus={:>#18x} mtvec={:>#18x} mepc={:>#18x}\n mcause={:>#18x} medeleg={:>#18x} mideleg={:>#18x}",
                    self.read(MSTATUS),
                    self.read(MTVEC),
                    self.read(MEPC),
                    self.read(MCAUSE),
                    self.read(MEDELEG),
                    self.read(MIDELEG),
                ),
                format!(
                    "sstatus={:>#18x} stvec={:>#18x} sepc={:>#18x}\n scause={:>#18x} sedeleg={:>#18x} sideleg={:>#18x}",
                    self.read(SSTATUS),
                    self.read(STVEC),
                    self.read(SEPC),
                    self.read(SCAUSE),
                    self.read(SEDELEG),
                    self.read(SIDELEG),
                ),
                format!(
                    "ustatus={:>#18x} utvec={:>#18x} uepc={:>#18x}\n ucause={:>#18x}",
                    self.read(USTATUS),
                    self.read(UTVEC),
                    self.read(UEPC),
                    self.read(UCAUSE),
                ),
            )
        )
    }
}

impl State {
    /// Create a new `state` object.
    pub fn new() -> Self {
        let mut csrs = [0; CSR_SIZE];
        let misa: u64 = (2 << 62) | // MXL[1:0]=2 (XLEN is 64)
            (1 << 18) | // Extensions[18] (Supervisor mode implemented)
            (1 << 12) | // Extensions[12] (Integer Multiply/Divide extension)
            (1 << 8) | // Extensions[8] (RV32I/64I/128I base ISA)
            (1 << 5) | // Extensions[5] (Single-precision floating-point extension)
            (1 << 3) | // Extensions[3] (Double-precision floating-point extension)
            (1 << 2) | // Extensions[2] (Compressed extension)
            1; // Extensions[0] (Atomic extension)
        csrs[MISA as usize] = misa;

        Self { csrs }
    }

    /// Increment the value in the TIME register.
    pub fn increment_time(&mut self) {
        self.csrs[TIME as usize] = self.csrs[TIME as usize].wrapping_add(1);
    }

    /// Read the val from the CSR.
    pub fn read(&self, addr: CsrAddress) -> u64 {
        // 4.1 Supervisor CSRs
        // "The supervisor should only view CSR state that should be visible to a supervisor-level
        // operating system. In particular, there is no information about the existence (or
        // non-existence) of higher privilege levels (machine level or other) visible in the CSRs
        // accessible by the supervisor.  Many supervisor CSRs are a subset of the equivalent
        // machine-mode CSR, and the machinemode chapter should be read first to help understand
        // the supervisor-level CSR descriptions."
        match addr {
            SSTATUS => {
                let mask = SSTATUS_SIE
                    | SSTATUS_SPIE
                    | SSTATUS_SPP
                    | SSTATUS_FS
                    | SSTATUS_XS
                    | SSTATUS_SUM
                    | SSTATUS_MXR
                    | SSTATUS_UXL;
                self.csrs[MSTATUS as usize] & mask
            }
            SIE => self.csrs[MIE as usize] & self.csrs[MIDELEG as usize],
            SIP => self.csrs[MIP as usize] & self.csrs[MIDELEG as usize],
            _ => self.csrs[addr as usize],
        }
    }

    /// Write the val to the CSR.
    pub fn write(&mut self, addr: CsrAddress, val: u64) {
        // 4.1 Supervisor CSRs
        // "The supervisor should only view CSR state that should be visible to a supervisor-level
        // operating system. In particular, there is no information about the existence (or
        // non-existence) of higher privilege levels (machine level or other) visible in the CSRs
        // accessible by the supervisor.  Many supervisor CSRs are a subset of the equivalent
        // machine-mode CSR, and the machinemode chapter should be read first to help understand
        // the supervisor-level CSR descriptions."
        match addr {
            MVENDORID => {}
            MARCHID => {}
            MIMPID => {}
            MHARTID => {}
            SSTATUS => {
                let mask = SSTATUS_SIE
                    | SSTATUS_SPIE
                    | SSTATUS_SPP
                    | SSTATUS_FS
                    | SSTATUS_XS
                    | SSTATUS_SUM
                    | SSTATUS_MXR;
                self.csrs[MSTATUS as usize] = (self.csrs[MSTATUS as usize] & !mask) | (val & mask);
            }
            SIE => {
                self.csrs[MIE as usize] = (self.csrs[MIE as usize] & !self.csrs[MIDELEG as usize])
                    | (val & self.csrs[MIDELEG as usize]);
            }
            SIP => {
                let mask = SSIP_BIT & self.csrs[MIDELEG as usize];
                self.csrs[MIP as usize] = (self.csrs[MIP as usize] & !mask) | (val & mask);
            }
            _ => self.csrs[addr as usize] = val,
        }
    }

    /// Read a bit from the CSR.
    pub fn read_bit(&self, addr: CsrAddress, bit: usize) -> u64 {
        if bit >= MXLEN {
            // TODO: raise exception?
        }

        if (self.read(addr) & (1 << bit)) != 0 {
            1
        } else {
            0
        }
    }

    /// Read a arbitrary length of bits from the CSR.
    pub fn read_bits<T: RangeBounds<usize>>(&self, addr: CsrAddress, range: T) -> u64 {
        let range = to_range(&range, MXLEN);

        if (range.start >= MXLEN) | (range.end > MXLEN) | (range.start >= range.end) {
            // TODO: ranse exception?
        }

        // Bitmask for high bits.
        let mut bitmask = 0;
        if range.end != 64 {
            bitmask = !0 << range.end;
        }

        // Shift away low bits.
        (self.read(addr) as u64 & !bitmask) >> range.start
    }

    /// Write a bit to the CSR.
    pub fn write_bit(&mut self, addr: CsrAddress, bit: usize, val: u64) {
        if bit >= MXLEN {
            // TODO: raise exception?
        }
        if val > 1 {
            // TODO: raise exception
        }

        if val == 1 {
            self.write(addr, self.read(addr) | 1 << bit);
        } else if val == 0 {
            self.write(addr, self.read(addr) & !(1 << bit));
        }
    }

    /// Write an arbitrary length of bits to the CSR.
    pub fn write_bits<T: RangeBounds<usize>>(&mut self, addr: CsrAddress, range: T, val: u64) {
        let range = to_range(&range, MXLEN);

        if (range.start >= MXLEN) | (range.end > MXLEN) | (range.start >= range.end) {
            // TODO: ranse exception?
        }

        let bitmask = (!0 << range.end) | !(!0 << range.start);
        // Set bits.
        self.write(addr, (self.read(addr) & bitmask) | (val << range.start))
    }

    /// Reset all the CSRs.
    pub fn reset(&mut self) {
        self.csrs = [0; CSR_SIZE];

        let misa: u64 = (2 << 62) | // MXL[1:0]=2 (XLEN is 64)
            (1 << 18) | // Extensions[18] (Supervisor mode implemented)
            (1 << 12) | // Extensions[12] (Integer Multiply/Divide extension)
            (1 << 8) | // Extensions[8] (RV32I/64I/128I base ISA)
            (1 << 5) | // Extensions[5] (Single-precision floating-point extension)
            (1 << 3) | // Extensions[3] (Double-precision floating-point extension)
            (1 << 2) | // Extensions[2] (Compressed extension)
            1; // Extensions[0] (Atomic extension)
        self.csrs[MISA as usize] = misa;
    }
}

/// Convert the val implement `RangeBounds` to the `Range` struct.
fn to_range<T: RangeBounds<usize>>(generic_range: &T, bit_length: usize) -> Range<usize> {
    let start = match generic_range.start_bound() {
        Bound::Excluded(&val) => val + 1,
        Bound::Included(&val) => val,
        Bound::Unbounded => 0,
    };
    let end = match generic_range.end_bound() {
        Bound::Excluded(&val) => val,
        Bound::Included(&val) => val + 1,
        Bound::Unbounded => bit_length,
    };

    start..end
}
