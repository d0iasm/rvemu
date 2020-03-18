use clap::{App, Arg};
use std::fs::File;
use std::io;
use std::io::prelude::*;

use rvemu_core::bus::DRAM_BASE;
use rvemu_core::cpu::Cpu;
use rvemu_core::emulator::Emulator;

/// Output current registers to the console.
fn dump_registers(cpu: &Cpu) {
    println!("---------------------------------------------------");
    println!("{}", cpu.xregs);
    println!("---------------------------------------------------");
    println!("{}", cpu.fregs);
    println!("---------------------------------------------------");
    println!("pc: {}", cpu.pc);
    println!("---------------------------------------------------");
    println!("{}", cpu.state)
}

/// Main function of RISC-V emulator for the CLI version.
fn main() -> io::Result<()> {
    let matches = App::new("rvemu: RISC-V emulator")
        .version("0.0.1")
        .author("Asami Doi <@d0iasm>")
        .arg(
            Arg::with_name("kernel")
                .short("k")
                .long("kernel")
                .takes_value(true)
                .required(true)
                .help("A kernel ELF image without headers"),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("A raw disk image"),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Enables to output debug messages"),
        )
        .get_matches();

    let mut kernel_file = File::open(
        &matches
            .value_of("kernel")
            .expect("failed to get a kernel file from a command option"),
    )?;
    let mut kernel_data = Vec::new();
    kernel_file.read_to_end(&mut kernel_data)?;

    let mut img_data = Vec::new();
    if let Some(img_file) = matches.value_of("file") {
        File::open(img_file)?.read_to_end(&mut img_data)?;
    }

    let mut emu = Emulator::new();
    emu.set_dram(kernel_data);
    emu.set_disk(img_data);
    emu.set_pc(DRAM_BASE);

    if matches.occurrences_of("debug") == 1 {
        emu.enable_debug();
    }

    emu.start();

    dump_registers(&emu.cpu);

    Ok(())
}
