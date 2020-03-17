//! The interrupt module contains all the interrupt kinds and the function to handle interrupts.

use crate::{
    cpu::{Cpu, Mode},
    csr::*,
};

/// All the interrupt kinds.
#[derive(Debug)]
pub enum Interrupt {
    UserExternalInterrupt,
    SupervisorExternalInterrupt,
    MachineExternalInterrupt,
}

impl Interrupt {
    /// Update CSRs and interrupt flags in devices.
    pub fn take_trap(&self, cpu: &mut Cpu) {
        let exception_code;
        let exception_pc = (cpu.pc as i64) - 4;

        match self {
            Interrupt::UserExternalInterrupt => {
                exception_code = 8;
            }
            Interrupt::SupervisorExternalInterrupt => {
                exception_code = 9;
            }
            Interrupt::MachineExternalInterrupt => {
                exception_code = 11;
            }
        }

        match cpu.mode {
            Mode::Machine => {
                cpu.state.write(MCAUSE, 1 << 63 | exception_code);
                cpu.state.write(MEPC, exception_pc);
            }
            Mode::Supervisor => {
                cpu.state.write(SCAUSE, 1 << 63 | exception_code);
                cpu.state.write(SEPC, exception_pc);
            }
            Mode::User => {
                cpu.state.write(UCAUSE, 1 << 63 | exception_code);
                cpu.state.write(UEPC, exception_pc);
            }
            _ => {}
        }
    }
}
