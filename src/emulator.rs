//! The emulator module represents an entire computer.

use crate::cpu::{Cpu, ExecCycle, REG_SP};
use crate::devices::dram::Dram;
//use crate::devices::rom::Rom;
use crate::devices::uart::Uart;
use crate::devices::virtio_blk::Virtio;
use crate::exception::Trap;
use std::sync::{Arc, Mutex};

// QEMU virt machine:
// https://github.com/qemu/qemu/blob/master/hw/riscv/virt.c#L46-L63

/// The address which the mask ROM starts.
pub const MROM_BASE: u64 = 0x1000;

/// The address which the core-local interruptor (CLINT) starts. It contains the timer and generates
/// per-hart software interrupts and timer interrupts.
pub const CLINT_BASE: u64 = 0x200_0000;

/// The address which the platform-level interrupt controller (PLIC) starts. The PLIC connects all
/// external interrupts in the system to all hart contexts in the system, via the external interrupt
/// source in each hart.
pub const PLIC_BASE: u64 = 0xc00_0000;

/// The address which UART starts. QEMU puts UART registers here in physical memory.
pub const UART_BASE: u64 = 0x1000_0000;
/// The size of UART.
pub const UART_SIZE: u64 = 0x100;

pub const UART_IRQ: u64 = 10;

/// The address which virtio starts.
pub const VIRTIO_BASE: u64 = 0x1000_1000;

pub const VIRTIO_IRQ: u64 = 1;

///
pub const DRAM_SIZE: u64 = 0x40000000;
/// The address which DRAM starts.
pub const DRAM_BASE: u64 = 0x8000_0000;

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
        let mut cpu = Cpu::new();
        let uart = Arc::new(Mutex::new(Uart::new(UART_IRQ)));
        cpu.bus.mount(UART_BASE, uart);
        Self {
            cpu: cpu,
            is_debug: false,
        }
    }

    /// Reset CPU state.
    pub fn reset(&mut self) {
        self.cpu.reset()
    }

    /// Set binary data to the beginning of the DRAM from the emulator console.
    pub fn initialize_dram(&mut self, data: Vec<u8>) {
        let dram = Arc::new(Mutex::new(Dram::new(DRAM_SIZE)));
        dram.lock().unwrap().initialize(data);
        self.cpu.bus.mount(DRAM_BASE, dram);
        self.initialize_sp(DRAM_BASE + DRAM_SIZE);
    }

    /// Set binary data to the virtio disk from the emulator console.
    pub fn initialize_disk(&mut self, data: Vec<u8>) {
        let disk = Arc::new(Mutex::new(Virtio::new(VIRTIO_IRQ)));
        disk.lock().unwrap().initialize(data);
        self.cpu.bus.mount(VIRTIO_BASE, disk);
    }

    /// Set the program counter to the CPU field.
    pub fn initialize_pc(&mut self, pc: u64) {
        self.cpu.pc = pc;
    }

    pub fn initialize_sp(&mut self, sp: u64) {
        self.cpu.xregs.write(REG_SP, sp);
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
                Ok(ExecCycle::Idle) => {
                    println!("Idle");
                    Trap::Requested
                }
                Ok(ExecCycle::Opcode(eop)) => {
                    println!("pc: {:#x}, inst: {:#x}", eop.pc, eop.opcode);
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
                Ok(ExecCycle::Idle) => {
                    println!("Idle");
                    Trap::Requested
                }
                Ok(ExecCycle::Opcode(eop)) => {
                    if self.is_debug {
                        let inst = eop.opcode;
                        println!(
                            "pc: {:#x}, inst: {:#x}, is_inst 16? {} pre_inst: {:#x}",
                            eop.pc,
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
