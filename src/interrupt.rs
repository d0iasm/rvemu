//! The interrupt module contains all the interrupt kinds and the function to handle interrupts.

use crate::{
    cpu::{Cpu, Mode},
    csr::*,
};

/// All the interrupt kinds.
#[derive(Debug)]
pub enum Interrupt {
    UserSoftwareInterrupt,
    SupervisorSoftwareInterrupt,
    MachineSoftwareInterrupt,
    UserTimerInterrupt,
    SupervisorTimerInterrupt,
    MachineTimerInterrupt,
    UserExternalInterrupt,
    SupervisorExternalInterrupt,
    MachineExternalInterrupt,
}

impl Interrupt {
    fn exception_code(&self) -> u64 {
        match self {
            Interrupt::UserSoftwareInterrupt => 0,
            Interrupt::SupervisorSoftwareInterrupt => 1,
            Interrupt::MachineSoftwareInterrupt => 3,
            Interrupt::UserTimerInterrupt => 4,
            Interrupt::SupervisorTimerInterrupt => 5,
            Interrupt::MachineTimerInterrupt => 7,
            Interrupt::UserExternalInterrupt => 8,
            Interrupt::SupervisorExternalInterrupt => 9,
            Interrupt::MachineExternalInterrupt => 11,
        }
    }

    /// Update CSRs and the program counter depending on an interrupt.
    pub fn take_trap(&self, cpu: &mut Cpu) {
        // 1.2 Privilege Levels
        // "Traps that increase privilege level are termed vertical traps, while traps that remain
        // at the same privilege level are termed horizontal traps."

        cpu.idle = false;

        let exception_pc = cpu.pc;
        let previous_mode = cpu.mode;
        let cause = self.exception_code();

        // 3.1.8 Machine Trap Delegation Registers (medeleg and mideleg)
        // "By default, all traps at any privilege level are handled in machine mode To increase
        // performance, implementations can provide individual read/write bits within medeleg and
        // mideleg to indicate that certain exceptions and interrupts should be processed directly
        // by a lower privilege level."
        //
        // "mideleg holds trap delegation bits for individual interrupts, with the layout of bits
        // matching those in the mip register (i.e., STIP interrupt delegation control is located
        // in bit 5)."
        // TODO: Why should a M-mode timer interrupt be taken in M-mode?
        if previous_mode <= Mode::Supervisor
            && ((cpu.state.read(MIDELEG) >> cause) & 1) == 1
            && cause != Interrupt::MachineTimerInterrupt.exception_code()
        {
            // Handle the trap in S-mode.
            cpu.mode = Mode::Supervisor;

            // Set the program counter to the supervisor trap-handler base address (stvec)
            // depending on the mode.
            let vector = match cpu.state.read_bit(STVEC, 0) {
                1 => 4 * cause, // vectored mode
                _ => 0,         // direct mode
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
            cpu.state.write(SCAUSE, 1 << 63 | cause);

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
                .write_sstatus(XSTATUS_SPIE, cpu.state.read_sstatus(XSTATUS_SIE));
            // Set a global interrupt-enable bit for supervisor mode (SIE, 1) to 0.
            cpu.state.write_sstatus(XSTATUS_SIE, 0);
            // 4.1.1 Supervisor Status Register (sstatus)
            // "When a trap is taken, SPP is set to 0 if the trap originated from user mode, or
            // 1 otherwise."
            match previous_mode {
                Mode::User => cpu.state.write_sstatus(XSTATUS_SPP, 0),
                _ => cpu.state.write_sstatus(XSTATUS_SPP, 1),
            }
        } else {
            // Handle the trap in M-mode.
            cpu.mode = Mode::Machine;

            // Set the program counter to the machine trap-handler base address (mtvec)
            // depending on the mode.
            let vector = match cpu.state.read_bit(MTVEC, 0) {
                1 => 4 * cause, // vectored mode
                _ => 0,         // direct mode
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
            cpu.state.write(MCAUSE, 1 << 63 | cause);

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

            // Set a previous interrupt-enable bit for machine mode (MPIE, 7) to the value
            // of a global interrupt-enable bit for machine mode (MIE, 3).
            cpu.state
                .write_mstatus(MSTATUS_MPIE, cpu.state.read_mstatus(MSTATUS_MIE));
            // Set a global interrupt-enable bit for machine mode (MIE, 3) to 0.
            cpu.state.write_mstatus(MSTATUS_MIE, 0);
            // When a trap is taken from privilege mode y into privilege mode x, xPIE is set
            // to the value of x IE; x IE is set to 0; and xPP is set to y.
            match previous_mode {
                Mode::User => cpu.state.write_mstatus(MSTATUS_MPP, Mode::User as u64),
                Mode::Supervisor => cpu
                    .state
                    .write_mstatus(MSTATUS_MPP, Mode::Supervisor as u64),
                Mode::Machine => cpu.state.write_mstatus(MSTATUS_MPP, Mode::Machine as u64),
                _ => panic!("previous privilege mode is invalid"),
            }
        }
    }
}
