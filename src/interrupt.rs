//! The interrupt module contains all the interrupt kinds and the function to handle interrupts.

use crate::{
    cpu::{Cpu, Mode},
    csr::*,
    devices::plic::PLIC_SCLAIM,
};

/// All the interrupt kinds.
#[derive(Debug)]
pub enum Interrupt {
    UserSoftwareInterrupt,
    SupervisorSoftwareInterrupt,
    MachineSoftwareInterrupt,
    UserExternalInterrupt(usize),
    SupervisorExternalInterrupt(usize),
    MachineExternalInterrupt(usize),
}

impl Interrupt {
    /// Update CSRs and interrupt flags in devices.
    pub fn take_trap(&self, cpu: &mut Cpu) {
        let mut claim = 0;
        let exception_code;
        let exception_pc = (cpu.pc as i64) - 4;

        match self {
            Interrupt::UserSoftwareInterrupt => {
                exception_code = 0;
            }
            Interrupt::SupervisorSoftwareInterrupt => {
                exception_code = 1;
            }
            Interrupt::MachineSoftwareInterrupt => {
                exception_code = 3;
            }
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

        match cpu.mode {
            Mode::Machine => {
                cpu.state.write(MCAUSE, 1 << 63 | exception_code);
                cpu.state.write(MEPC, exception_pc);
                // TODO: handle mode? but xv6 seems not to care about mode.
                cpu.pc = cpu.state.read(MTVEC) as usize;
            }
            Mode::Supervisor => {
                cpu.state.write(SCAUSE, 1 << 63 | exception_code);
                cpu.state.write(SEPC, exception_pc);

                // TODO: is it correct?
                // Set a global interrupt-enable bit for supervisor mode (SIE, 1) to 0.
                cpu.state.write_bit(SSTATUS, 1, false);
                // Set a privious privilege mode for supervisor mode (SPP, 8) to 0.
                cpu.state.write_bit(SSTATUS, 8, true);
                // TODO: handle mode? but xv6 seems not to care about mode.
                cpu.pc = cpu.state.read(STVEC) as usize;
            }
            Mode::User => {
                cpu.state.write(UCAUSE, 1 << 63 | exception_code);
                cpu.state.write(UEPC, exception_pc);
                // TODO: handle mode? but xv6 seems not to care about mode.
                cpu.pc = cpu.state.read(UTVEC) as usize;
            }
            _ => {}
        }
    }
}
