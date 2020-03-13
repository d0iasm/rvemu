//! The emulator module represents an entire computer.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

use crate::{bus::DRAM_BASE, cpu::Cpu};

/// Emulator struct to holds a CPU.
pub struct Emulator {
    pub cpu: Arc<Mutex<Cpu>>,
}

impl Emulator {
    /// Constructor for an emulator.
    pub fn new() -> Emulator {
        Self {
            cpu: Arc::new(Mutex::new(Cpu::new())),
        }
    }

    /// Reset CPU state.
    pub fn reset(&mut self) {
        let mut cpu = self.cpu.lock().expect("failed to get a CPU object");
        cpu.reset()
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
        let size = cpu.bus.dram_size();
        drop(cpu);

        // TODO: delete `count` variable bacause it's for debug.
        let mut count = 0;

        loop {
            // TODO: Delete the following sleep function. This is for debug.
            //thread::sleep(time::Duration::from_millis(500));

            if let Ok(mut cpu) = self.cpu.try_lock() {
                // 1. Fetch.
                let data_or_error = cpu.fetch();

                dbg!(format!(
                    "pc: {} , data: {:#?}",
                    cpu.pc,
                    &data_or_error,
                ));

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

                // TODO: Delete this count because it's for debug.
                count += 1;

                // TODO: reconsider the termination condition.
                if result.is_err() | (cpu.pc >= size + DRAM_BASE + 0x1000) | (count > 1000000) {
                    //if result.is_err() | (cpu.pc >= size + DRAM_BASE + 0x1000) {
                    dbg!(format!(
                        "pc: {}, count: {}, result {:#?}",
                        cpu.pc, count, result
                    ));
                    return;
                }
            }
        }
    }
}
