pub mod stdio;
mod utils;

use rvemu_core::emulator;

use wasm_bindgen::prelude::*;

use stdio::*;

/// Wrapper for rvemu::emulator::Emulator to connect to WebAssembly.
#[wasm_bindgen]
pub struct Emulator {
    emu: emulator::Emulator,
}

#[wasm_bindgen]
impl Emulator {
    /// Constructor for the emulator.
    pub fn new() -> Emulator {
        // Initialize for putting error messages to a browser console.
        utils::set_panic_hook();

        Self {
            emu: emulator::Emulator::new(),
        }
    }

    /// Reset the emulator.
    pub fn reset(&mut self) {
        self.emu.reset();
    }

    /// Set binary data to the beginning of the DRAM from the emulator console of a browser.
    pub fn set_dram(&mut self, data: Vec<u8>) {
        self.emu.set_dram(data);
    }

    /// Start executing.
    pub fn start(&mut self) {
        self.emu.start(stdin, stdout);
    }

    /// Output current registers.
    pub fn dump_registers(&self) {
        let cpu = self.emu.cpu.lock().expect("failed to get a CPU object");
        stdout_log(&format!("{}", cpu.xregs));
        stdout_log(&format!(
            "---------------------------------------------------"
        ));
        stdout_log(&format!("{}", cpu.fregs));
        stdout_log(&format!(
            "---------------------------------------------------"
        ));
        stdout_log(&format!("pc: {}", cpu.pc));
    }
}
