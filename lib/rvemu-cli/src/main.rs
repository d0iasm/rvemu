pub mod stdio;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use rvemu_core::bus::*;
use rvemu_core::cpu::*;
use rvemu_core::emulator::Emulator;

use stdio::*;

/// Output current registers to the console.
fn dump_registers(cpu: &Cpu) {
    println!("{}", cpu.xregs);
    println!("---------------------------------------------------");
    println!("{}", cpu.fregs);
    println!("---------------------------------------------------");
    println!("pc: {}", cpu.pc);
}

/// Main function of RISC-V emulator for the CLI version.
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: ./rvemu-cli <binary-file-name>");
    }

    let mut file = File::open(&args[1])?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let mut emu = Emulator::new();
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    emu.start(stdin);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        dump_registers(&cpu);
    }

    Ok(())
}
