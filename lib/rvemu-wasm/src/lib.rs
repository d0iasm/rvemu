pub mod stdio;
mod utils;

use rvemu_core::bus::*;
use rvemu_core::cpu::*;

use wasm_bindgen::prelude::*;

use stdio::*;

/// An emulator struct to holds a CPU and a bus.
#[wasm_bindgen]
pub struct Emulator {
    cpu: Cpu,
    bus: Bus,
}

#[wasm_bindgen]
impl Emulator {
    /// Constructor for an emulator.
    pub fn new() -> Emulator {
        // Initialize for putting error messages to a browser console.
        utils::set_panic_hook();

        Emulator {
            cpu: Cpu::new(),
            bus: Bus::new(),
        }
    }

    /// Reset CPU state.
    pub fn reset(&mut self) {
        self.cpu.reset()
    }

    /// Set binary data to the beginning of the DRAM from the emulator console of a browser.
    pub fn set_dram(&mut self, data: Vec<u8>) {
        //let header = Elf64Ehdr::new(&bin);
        //if !header.verify() {
        //stdout_log(&format!("unexpected ELF format"))
        //}
        //log(&format!("{:#?}", header));
        // Set an entry point. Divide 8 because `e_entry` is the number of bits.
        //self.cpu.pc = header.e_entry as usize;

        self.bus.set_dram(data);
    }

    /// Start executing.
    pub fn start(&mut self) {
        self.cpu.start(&mut self.bus, stdin);
    }

    /// Output current registers.
    pub fn dump_registers(&self) {
        stdout_log(&format!("{}", self.cpu.xregs));
        stdout_log(&format!(
            "---------------------------------------------------"
        ));
        stdout_log(&format!("{}", self.cpu.fregs));
        stdout_log(&format!(
            "---------------------------------------------------"
        ));
        stdout_log(&format!("pc: {}", self.cpu.pc));
    }
}
