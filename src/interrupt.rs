//! The interrupt module contains all the interrupt kinds and the function to handle interrupts.

use crate::{
    cpu::{Cpu, Mode},
    csr::*,
    devices::plic::PLIC_SCLAIM,
};

/// All the interrupt kinds.
#[derive(Debug)]
pub enum Interrupt {
    UserExternalInterrupt(usize),
    SupervisorExternalInterrupt(usize),
    MachineExternalInterrupt(usize),
}

impl Interrupt {
    /// Update CSRs and interrupt flags in devices.
    pub fn take_trap(&self, cpu: &mut Cpu) {
        let claim;
        let exception_code;
        let exception_pc = (cpu.pc as i64) - 4;

        match self {
            Interrupt::UserExternalInterrupt(irq) => {
                claim = *irq;
                exception_code = 8;
            }
            Interrupt::SupervisorExternalInterrupt(irq) => {
                claim = *irq;
                exception_code = 9;
            }
            Interrupt::MachineExternalInterrupt(irq) => {
                claim = *irq;
                exception_code = 11;
            }
        }

        // TODO: hart is 0, and write a value to MCLAIM if the mode is machine.
        cpu.bus
            .write32(PLIC_SCLAIM, claim as u32)
            .expect("failed to write an IRQ to the PLIC_SCLAIM");

        /*
        // TODO: handle mode?
        cpu.pc = cpu.state.read_bits(MTVEC, 2..) as usize;
        */

        match cpu.mode {
            Mode::Machine => {
                cpu.state.write(MCAUSE, 1 << 63 | exception_code);
                cpu.state.write(MEPC, exception_pc);
                // TODO: handle mode?
                cpu.pc = cpu.state.read_bits(MTVEC, 2..) as usize;
            }
            Mode::Supervisor => {
                cpu.state.write(SCAUSE, 1 << 63 | exception_code);
                cpu.state.write(SEPC, exception_pc);
                // TODO: handle mode?
                cpu.pc = cpu.state.read_bits(STVEC, 2..) as usize;
            }
            Mode::User => {
                cpu.state.write(UCAUSE, 1 << 63 | exception_code);
                cpu.state.write(UEPC, exception_pc);
                // TODO: handle mode?
                cpu.pc = cpu.state.read_bits(UTVEC, 2..) as usize;
            }
            _ => {}
        }
    }
}
