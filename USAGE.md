RISC-V emulator core implementation.

*NOTE: This project is currently under intensely development. The source code
might be changed dramatically.*

# How to use
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

    let mut emu = Emulator::new(); // Create an emulator object.
    emu.set_dram(data); // Place the binary data in the beginning of DRAM.
    emu.set_pc(DRAM_BASE); // Set the program counter to 0x8000_0000, which is the
    address DRAM starts.
    emu.start();
    
    assert_eq!();
}
```

See the example usage in
[rvemu/lib/rvemu-cli/src/main.rs](https://github.com/d0iasm/rvemu/blob/master/lib/rvemu-cli/src/main.rs).

# Features
Supports the following features:
- RV64G ISAs
- Previleged ISAs
- Previleged mode
- CSRs 
- Exceptions
- Devices (DRAM, UART, CLINT) 
