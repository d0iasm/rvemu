//! The emulator module represents an entire computer.

use std::sync::{Arc, Mutex};

use crate::cpu::Cpu;

/// Emulator struct to holds a CPU.
pub struct Emulator {
    /// The CPU which is the core implementation of this emulator.
    pub cpu: Arc<Mutex<Cpu>>,
    /// The debug flag. Output messages if it's true, otherwise output nothing.
    is_debug: bool,
}

impl Emulator {
    /// Constructor for an emulator.
    pub fn new() -> Emulator {
        Self {
            cpu: Arc::new(Mutex::new(Cpu::new())),
            is_debug: false,
        }
    }

    /// Reset CPU state.
    pub fn reset(&mut self) {
        let mut cpu = self.cpu.lock().expect("failed to get a CPU object");
        cpu.reset()
    }

    /// Enable the debug flag.
    pub fn enable_debug(&mut self) {
        self.is_debug = true;
    }

    /// Set binary data to the beginning of the DRAM from the emulator console of a browser.
    pub fn set_dram(&mut self, data: Vec<u8>) {
        let mut cpu = self.cpu.lock().expect("failed to get a CPU object");
        cpu.bus.set_dram(data);
    }

    /// Set the program counter to the CPU field.
    pub fn set_pc(&mut self, pc: usize) {
        let mut cpu = self.cpu.lock().expect("failed to get a CPU object");
        cpu.pc = pc;
    }

    /// Start executing the emulator.
    pub fn start(&mut self) {
        let cpu = self.cpu.lock().expect("failed to get a CPU object");
        drop(cpu);

        loop {
            if let Ok(mut cpu) = self.cpu.try_lock() {
                // 1. Fetch.
                let data_or_error = cpu.fetch();

                if self.is_debug {
                    dbg!(format!("pc: {} , data: {:#?}", cpu.pc, &data_or_error));
                }

                // 2. Add 4 to the program counter.
                cpu.pc += 4;

                // 3. Decode.
                // 4. Execution.
                let result = match data_or_error {
                    Ok(data) => match cpu.execute(data) {
                        Ok(_) => Ok(()),
                        Err(error) => error.take_trap(&mut cpu),
                    },
                    Err(error) => error.take_trap(&mut cpu),
                };

                // TODO: Take interrupts.

                if result.is_err() {
                    dbg!(format!(
                        "pc: {}, result {:#?}",
                        cpu.pc, result
                    ));
                    return;
                }
            }
        }
    }
}
