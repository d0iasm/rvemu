# RISC-V emulator core implementation
[![Build Status](https://travis-ci.com/d0iasm/rvemu.svg?branch=master)](https://travis-ci.com/d0iasm/rvemu)
[![Actions Status](https://github.com/d0iasm/rvemu/workflows/CI/badge.svg)](https://github.com/d0iasm/rvemu/actions)
[![docs.rs](https://docs.rs/rvemu/badge.svg)](https://docs.rs/rvemu)
[![crate.io](https://img.shields.io/crates/v/rvemu.svg)](https://crates.io/crates/rvemu)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/d0iasm/rvemu/master/LICENSE)

*NOTE: This project is currently under intensely development. The source code
might be changed dramatically.*

## How to use
Create an `Emulator` object, place a binary data in DRAM and set the program counter to
`DRAM_BASE`. The binary data must contain no headers for now. The example is here:
```rust
use rvemu::bus::DRAM_BASE;
use rvemu::emulator::Emulator;

fn main() {
    // Create a dummy binary data.
    let data = vec![
        0x93, 0x0f, 0xa0, 0x02, // addi x31, x0, 42
    ];

    // Create an emulator object.
    let mut emu = Emulator::new();
    // Place the binary data in the beginning of DRAM.
    emu.set_dram(data);
    // Set the program counter to 0x8000_0000, which is the address DRAM starts.
    emu.set_pc(DRAM_BASE);
    // Start the emulator.
    emu.start();

    // `IllegalInstruction` is raised for now because of the termination condition of the emulator,
    // but the register is successfully updated.
    assert_eq!(42, emu.cpu.xregs.read(31));
}
```

See the example usage in
[rvemu/lib/rvemu-cli/src/main.rs](https://github.com/d0iasm/rvemu/blob/master/lib/rvemu-cli/src/main.rs).

## Features
Now, supports the following features (will be added in the future):
- RV64G ISAs
- Privileged ISAs
- Control and status registers (CSRs)
- Virtual memory system (Sv39)
- Devices
  - UART: universal asynchronous receiver-transmitter
  - CLINT: core local interruptor
  - PLIC: platform level interrupt controller
  - Virtio
