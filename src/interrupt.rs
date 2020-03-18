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
    fn exception_code(&self) -> i64 {
        match self {
            Interrupt::UserSoftwareInterrupt => 0,
            Interrupt::SupervisorSoftwareInterrupt => 1,
            Interrupt::MachineSoftwareInterrupt => 3,
            Interrupt::UserExternalInterrupt(_irq) => 8,
            Interrupt::SupervisorExternalInterrupt(_irq) => 9,
            Interrupt::MachineExternalInterrupt(_irq) => 11,
        }
    }

    fn irq(&self) -> u32 {
        match self {
            Interrupt::UserExternalInterrupt(irq) => *irq as u32,
            Interrupt::SupervisorExternalInterrupt(irq) => *irq as u32,
            Interrupt::MachineExternalInterrupt(irq) => *irq as u32,
            _ => 0,
        }
    }

    /// Update CSRs and interrupt flags in devices.
    pub fn take_trap(&self, cpu: &mut Cpu) {
        let exception_pc = cpu.pc as i64;

        // TODO: assume that hart is 0
        // TODO: write a value to MCLAIM if the mode is machine
        cpu.bus
            .write32(PLIC_SCLAIM, self.irq())
            .expect("failed to write an IRQ to the PLIC_SCLAIM");

        match cpu.mode {
            Mode::Machine => {
                // Set the program counter to the machine trap-handler base address (mtvec)
                // depending on the mode.
                let vector = match cpu.state.read_bit(MTVEC, 0) {
                    true => 4 * self.exception_code(), // vectored mode
                    false => 0,                        // direct mode
                };
                cpu.pc = ((cpu.state.read(MTVEC) & !1) + vector) as usize;

                cpu.state.write(MCAUSE, 1 << 63 | self.exception_code());
                cpu.state.write(MEPC, exception_pc);

                // Set a privious interrupt-enable bit for supervisor mode (MPIE, 7) to the value
                // of a global interrupt-enable bit for supervisor mode (MIE, 3).
                cpu.state
                    .write_bit(MSTATUS, 7, cpu.state.read_bit(MSTATUS, 3));
                // Set a global interrupt-enable bit for supervisor mode (MIE, 3) to 0.
                cpu.state.write_bit(MSTATUS, 3, false);
                // Set a privious privilege mode for supervisor mode (MPP, 11..13) to 0.
                cpu.state.write_bits(MSTATUS, 11..13, 0b00);
            }
            Mode::Supervisor => {
                // Set the program counter to the machine trap-handler base address (mtvec)
                // depending on the mode.
                let vector = match cpu.state.read_bit(STVEC, 0) {
                    true => 4 * self.exception_code(), // vectored mode
                    false => 0,                        // direct mode
                };
                cpu.pc = ((cpu.state.read(STVEC) & !1) + vector) as usize;

                cpu.state.write(SCAUSE, 1 << 63 | self.exception_code());
                cpu.state.write(SEPC, exception_pc);

                // Set a privious interrupt-enable bit for supervisor mode (SPIE, 5) to the value
                // of a global interrupt-enable bit for supervisor mode (SIE, 1).
                cpu.state
                    .write_bit(SSTATUS, 5, cpu.state.read_bit(SSTATUS, 1));
                // Set a global interrupt-enable bit for supervisor mode (SIE, 1) to 0.
                cpu.state.write_bit(SSTATUS, 1, false);
                // Set a privious privilege mode for supervisor mode (SPP, 8) to 0.
                cpu.state.write_bit(SSTATUS, 8, true);
            }
            Mode::User => {
                cpu.state.write(UCAUSE, 1 << 63 | self.exception_code());
                cpu.state.write(UEPC, exception_pc);
                // TODO: handle mode? but xv6 seems not to care about mode.
                cpu.pc = cpu.state.read(UTVEC) as usize;
            }
            _ => {}
        }
    }
}
