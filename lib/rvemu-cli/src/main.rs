use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use rvemu_core::cpu::*;
use rvemu_core::memory::*;

fn dump_registers(cpu: &Cpu) {
    println!("{}", cpu.xregs);
    println!("---------------------------------------------------");
    println!("{}", cpu.fregs);
    println!("---------------------------------------------------");
    println!("pc: {}", cpu.pc);
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: ./rvemu-cli <binary-file-name>");
    }

    let mut file = File::open(&args[1])?;
    let mut dram = Vec::new();
    file.read_to_end(&mut dram)?;

    let mut cpu = Cpu::new();
    let mut mem = Memory { dram };

    cpu.start(&mut mem);

    dump_registers(&cpu);

    Ok(())
}
