//! RISC-V emulator core implementation.
//!
//! # How to use
//! Create an `Emulator` object, place a binary data in DRAM and set the program counter to
//! `DRAM_BASE`. The binary data must contain no headers for now. The example is here:
//! ```rust
//! use rvemu::bus::DRAM_BASE;
//! use rvemu::emulator::Emulator;
//!
//! fn main() {
//!     // Create a dummy binary data.
//!     let data = vec![
//!         0x93, 0x0f, 0xa0, 0x02, // addi x31, x0, 42
//!     ];
//!
//!     // Create an emulator object.
//!     let mut emu = Emulator::new();
//!     // Place the binary data in the beginning of DRAM.
//!     emu.initialize_dram(data);
//!     // Set the program counter to 0x8000_0000, which is the address DRAM starts.
//!     emu.initialize_pc(DRAM_BASE);
//!     // Start the emulator.
//!     emu.start();
//!
//!     // `IllegalInstruction` is raised for now because of the termination condition of the emulator,
//!     // but the register is successfully updated.
//!     assert_eq!(42, emu.cpu.xregs.read(31));
//! }
//! ```
//!
//! See the example usage in
//! [rvemu/lib/rvemu-cli/src/main.rs](https://github.com/d0iasm/rvemu/blob/master/lib/rvemu-cli/src/main.rs).

pub mod bus;
pub mod cpu;
pub mod csr;
pub mod devices;
pub mod dram;
pub mod emulator;
pub mod exception;
pub mod interrupt;
pub mod rom;
