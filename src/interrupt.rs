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
    UserExternalInterrupt(u64),
    SupervisorExternalInterrupt(u64),
    MachineExternalInterrupt(u64),
}

impl Interrupt {
    fn exception_code(&self) -> u64 {
        match self {
            Interrupt::UserSoftwareInterrupt => 0,
            Interrupt::SupervisorSoftwareInterrupt => 1,
            Interrupt::MachineSoftwareInterrupt => 3,
            Interrupt::UserExternalInterrupt(_irq) => 8,
            Interrupt::SupervisorExternalInterrupt(_irq) => 9,
            Interrupt::MachineExternalInterrupt(_irq) => 11,
        }
    }

    fn irq(&self) -> u64 {
        match self {
            Interrupt::UserExternalInterrupt(irq) => *irq,
            Interrupt::SupervisorExternalInterrupt(irq) => *irq,
            Interrupt::MachineExternalInterrupt(irq) => *irq,
            _ => 0,
        }
    }

    /// Update CSRs and interrupt flags in devices.
    pub fn take_trap(&self, cpu: &mut Cpu) {
        let exception_pc = cpu.pc;
        let prev_mode = cpu.mode;

        let mideleg = cpu.state.read(MIDELEG);
        let sideleg = cpu.state.read(SIDELEG);
        let pos = (self.exception_code() | 1 << 63) & 0xffff;
        match ((mideleg >> pos) & 1) == 0 {
            true => cpu.mode = Mode::Machine,
            false => match ((sideleg >> pos) & 1) == 0 {
                true => cpu.mode = Mode::Supervisor,
                false => cpu.mode = Mode::User,
            },
        }

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
                cpu.pc = ((cpu.state.read(MTVEC) & !1) + vector) as u64;

                // 3.1.15 Machine Exception Program Counter (mepc)
                // "The low bit of mepc (mepc[0]) is always zero."
                // "When a trap is taken into M-mode, mepc is written with the virtual address of
                // the instruction that was interrupted or that encountered the exception.
                // Otherwise, mepc is never written by the implementation, though it may be
                // explicitly written by software."
                cpu.state.write(MEPC, exception_pc & !1);

                // 3.1.16 Machine Cause Register (mcause)
                // "When a trap is taken into M-mode, mcause is written with a code indicating
                // the event that caused the trap. Otherwise, mcause is never written by the
                // implementation, though it may be explicitly written by software."
                cpu.state.write(MCAUSE, 1 << 63 | self.exception_code());

                // 3.1.17 Machine Trap Value (mtval) Register
                // "When a trap is taken into M-mode, mtval is either set to zero or written with
                // exception-specific information to assist software in handling the trap.
                // Otherwise, mtval is never written by the implementation, though it may be
                // explicitly written by software."
                // "When a hardware breakpoint is triggered, or an instruction-fetch, load, or
                // store address-misaligned, access, or page-fault exception occurs, mtval is
                // written with the faulting virtual address. On an illegal instruction trap,
                // mtval may be written with the first XLEN or ILEN bits of the faulting
                // instruction as described below. For other traps, mtval is set to zero."
                cpu.state.write(MTVAL, 0);

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
                // Set the program counter to the machine trap-handler base address (stvec)
                // depending on the mode.
                let vector = match cpu.state.read_bit(STVEC, 0) {
                    true => 4 * self.exception_code(), // vectored mode
                    false => 0,                        // direct mode
                };
                cpu.pc = ((cpu.state.read(STVEC) & !1) + vector) as u64;

                // 4.1.9 Supervisor Exception Program Counter (sepc)
                // "The low bit of sepc (sepc[0]) is always zero."
                // "When a trap is taken into S-mode, sepc is written with the virtual address of
                // the instruction that was interrupted or that encountered the exception.
                // Otherwise, sepc is never written by the implementation, though it may be
                // explicitly written by software."
                cpu.state.write(SEPC, exception_pc & !1);

                // 4.1.10 Supervisor Cause Register (scause)
                // "When a trap is taken into S-mode, scause is written with a code indicating
                // the event that caused the trap.  Otherwise, scause is never written by the
                // implementation, though it may be explicitly written by software."
                cpu.state.write(SCAUSE, 1 << 63 | self.exception_code());

                // 4.1.11 Supervisor Trap Value (stval) Register
                // "When a trap is taken into S-mode, stval is written with exception-specific
                // information to assist software in handling the trap. Otherwise, stval is never
                // written by the implementation, though it may be explicitly written by software."
                // "When a hardware breakpoint is triggered, or an instruction-fetch, load, or
                // store address-misaligned, access, or page-fault exception occurs, stval is
                // written with the faulting virtual address. On an illegal instruction trap,
                // stval may be written with the first XLEN or ILEN bits of the faulting
                // instruction as described below. For other exceptions, stval is set to zero."
                cpu.state.write(STVAL, 0);

                // Set a privious interrupt-enable bit for supervisor mode (SPIE, 5) to the value
                // of a global interrupt-enable bit for supervisor mode (SIE, 1).
                cpu.state
                    .write_bit(SSTATUS, 5, cpu.state.read_bit(SSTATUS, 1));
                // Set a global interrupt-enable bit for supervisor mode (SIE, 1) to 0.
                cpu.state.write_bit(SSTATUS, 1, false);
                // 4.1.1 Supervisor Status Register (sstatus)
                // "When a trap is taken, SPP is set to 0 if the trap originated from user mode, or
                // 1 otherwise."
                match prev_mode {
                    Mode::User => cpu.state.write_bit(SSTATUS, 8, false),
                    _ => cpu.state.write_bit(SSTATUS, 8, true),
                }
            }
            Mode::User => {
                // Set the program counter to the machine trap-handler base address (utvec)
                // depending on the mode.
                let vector = match cpu.state.read_bit(UTVEC, 0) {
                    true => 4 * self.exception_code(), // vectored mode
                    false => 0,                        // direct mode
                };
                cpu.pc = ((cpu.state.read(UTVEC) & !1) + vector) as u64;

                cpu.state.write(UCAUSE, 1 << 63 | self.exception_code());
                cpu.state.write(UEPC, exception_pc);
                cpu.state.write(UTVAL, exception_pc);

                // TODO: implement to update USTATUS
            }
            _ => {}
        }
    }
}
