//! The emulator module represents an entire computer.

use crate::cpu::Cpu;
use crate::exception::Trap;

/// The emulator to hold a CPU.
pub struct Emulator {
    /// The CPU which is the core implementation of this emulator.
    pub cpu: Cpu,
    /// The debug flag. Output messages if it's true, otherwise output nothing.
    pub is_debug: bool,
    /// The test flag for riscv/riscv-tests.
    pub is_test: bool,
}

impl Emulator {
    /// Constructor for an emulator.
    pub fn new() -> Emulator {
        Self {
            cpu: Cpu::new(),
            is_debug: false,
            is_test: false,
        }
    }

    /// Reset CPU state.
    pub fn reset(&mut self) {
        self.cpu.reset()
    }

    /// Set binary data to the beginning of the DRAM from the emulator console.
    pub fn initialize_dram(&mut self, data: Vec<u8>) {
        self.cpu.bus.initialize_dram(data);
    }

    /// Set binary data to the virtio disk from the emulator console.
    pub fn initialize_disk(&mut self, data: Vec<u8>) {
        self.cpu.bus.initialize_disk(data);
    }

    /// Set the program counter to the CPU field.
    pub fn initialize_pc(&mut self, pc: u64) {
        self.cpu.pc = pc;
    }

    /// Start executing the emulator.
    pub fn start(&mut self) {
        let mut count = 0;
        loop {
            count += 1;
            // This is a workaround for unit tests to finish the execution.
            if self.is_test && count > 1000 {
                return;
            }
            if self.cpu.is_count && count > 50000000 {
                return;
            }

            // Run a cycle on peripheral devices.
            self.cpu.devices_increment();

            // Take an interrupt.
            match self.cpu.check_pending_interrupt() {
                Some(interrupt) => interrupt.take_trap(&mut self.cpu),
                None => {}
            }

            // Execute a fetched instruction.
            let trap = match self.cpu.execute() {
                Ok(inst) => {
                    if self.is_debug {
                        dbg!(format!(
                            "pc: {:#x} , inst: {:#x}, is_inst 16? {}",
                            self.cpu.pc,
                            inst,
                            // Check if an instruction is one of the compressed instructions.
                            (inst & 0xffff_0000) == 0,
                        ));
                    }
                    // Return a dummy trap.
                    Trap::Requested
                }
                Err(exception) => exception.take_trap(&mut self.cpu),
            };

            match trap {
                Trap::Fatal => {
                    println!("pc: {:#x}, trap {:#?}", self.cpu.pc, trap);
                    return;
                }
                _ => {}
            }
        }
    }
}
