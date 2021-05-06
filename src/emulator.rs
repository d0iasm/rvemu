//! The emulator module represents an entire computer.

use crate::cpu::Cpu;
use crate::exception::Trap;

/// The emulator to hold a CPU.
pub struct Emulator {
    /// The CPU which is the core implementation of this emulator.
    pub cpu: Cpu,
    /// The debug flag. Output messages if it's true, otherwise output nothing.
    pub is_debug: bool,
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

    /// Start executing the emulator with limited range of program. This method is for test.
    /// No interrupts happen.
    pub fn test_start(&mut self, start: u64, end: u64) {
        println!("----- test start -----");
        let mut count = 0;
        loop {
            count += 1;
            if self.cpu.pc < start || end <= self.cpu.pc {
                return;
            }
            // This is a workaround for unit tests to finish the execution.
            if count > 1000 {
                return;
            }

            match self.cpu.execute() {
                Ok(inst) => {
                    println!("pc: {:#x}, inst: {:#x}", self.cpu.pc.wrapping_sub(4), inst);
                    Trap::Requested
                }
                Err(exception) => {
                    println!("pc: {:#x}, exception: {:?}", self.cpu.pc, exception);
                    exception.take_trap(&mut self.cpu)
                }
            };
        }
    }

    /// Start executing the emulator for debug.
    fn debug_start(&mut self) {
        let mut count = 0;
        loop {
            count += 1;
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

            // Execute an instruction.
            let trap = match self.cpu.execute() {
                Ok(inst) => {
                    if self.is_debug {
                        println!(
                            "pc: {:#x}, inst: {:#x}, is_inst 16? {} pre_inst: {:#x}",
                            self.cpu.pc.wrapping_sub(4),
                            inst,
                            // Check if an instruction is one of the compressed instructions.
                            inst & 0b11 == 0 || inst & 0b11 == 1 || inst & 0b11 == 2,
                            self.cpu.pre_inst,
                        );
                    }
                    // Return a placeholder trap.
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

    /// Start executing the emulator.
    pub fn start(&mut self) {
        if self.is_debug || self.cpu.is_count {
            self.debug_start();
        }

        loop {
            // Run a cycle on peripheral devices.
            self.cpu.devices_increment();

            // Take an interrupt.
            match self.cpu.check_pending_interrupt() {
                Some(interrupt) => interrupt.take_trap(&mut self.cpu),
                None => {}
            }

            // Execute an instruction.
            let trap = match self.cpu.execute() {
                Ok(_) => {
                    // Return a placeholder trap.
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
