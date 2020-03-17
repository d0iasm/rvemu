//! The emulator module represents an entire computer.

use crate::cpu::Cpu;

/// Emulator struct to holds a CPU.
pub struct Emulator {
    /// The CPU which is the core implementation of this emulator.
    pub cpu: Cpu,
    /// The debug flag. Output messages if it's true, otherwise output nothing.
    is_debug: bool,
}

impl Emulator {
    /// Constructor for an emulator.
    pub fn new() -> Emulator {
        Self {
            cpu: Cpu::new(),
            is_debug: false,
        }
    }

    /// Reset CPU state.
    pub fn reset(&mut self) {
        self.cpu.reset()
    }

    /// Enable the debug flag.
    pub fn enable_debug(&mut self) {
        self.is_debug = true;
    }

    /// Set binary data to the beginning of the DRAM from the emulator console of a browser.
    pub fn set_dram(&mut self, data: Vec<u8>) {
        self.cpu.bus.set_dram(data);
    }

    /// Set the program counter to the CPU field.
    pub fn set_pc(&mut self, pc: usize) {
        self.cpu.pc = pc;
    }

    /// Start executing the emulator.
    pub fn start(&mut self) {
        loop {
            // 1. Fetch.
            let data_or_error = self.cpu.fetch();

            if self.is_debug {
                dbg!(format!("pc: {} , data: {:#?}", self.cpu.pc, &data_or_error));
            }

            // 2. Add 4 to the program counter.
            self.cpu.pc += 4;

            // 3. Decode.
            // 4. Execution.
            let result = match data_or_error {
                Ok(data) => match self.cpu.execute(data) {
                    Ok(_) => Ok(()),
                    Err(exception) => exception.take_trap(&mut self.cpu),
                },
                Err(exception) => exception.take_trap(&mut self.cpu),
            };

            match self.cpu.check_interrupt() {
                Some(interrupt) => interrupt.take_trap(&mut self.cpu),
                None => {}
            }

            if result.is_err() {
                dbg!(format!("pc: {}, result {:#?}", self.cpu.pc, result));
                return;
            }
        }
    }
}
