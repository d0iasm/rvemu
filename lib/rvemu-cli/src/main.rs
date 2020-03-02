use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use rvemu_core::cpu::*;
use rvemu_core::memory::*;

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
    let mut dram = Vec::new();
    file.read_to_end(&mut dram)?;

    let mut cpu = Cpu::new();
    let mut mem = Memory::new();
    for i in 0..dram.len() {
        mem.dram[i] = dram[i];
    }

    cpu.start(&mut mem);

    dump_registers(&cpu);

    Ok(())
}
