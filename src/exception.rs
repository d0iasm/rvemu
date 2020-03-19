//! The exception module contains all the exception kinds and the function to handle exceptions.

use crate::{
    cpu::{Cpu, Mode},
    csr::*,
};

/// All the exception kinds.
#[derive(Debug)]
pub enum Exception {
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction(String),
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAMOAddressMisaligned,
    StoreAMOAccessFault,
    EnvironmentCallFromUMode,
    EnvironmentCallFromSMode,
    EnvironmentCallFromMMode,
    InstructionPageFault,
    LoadPageFault,
    StoreAMOPageFault,
}

impl Exception {
    fn exception_code(&self) -> i64 {
        match self {
            Exception::InstructionAddressMisaligned => 0,
            Exception::InstructionAccessFault => 1,
            Exception::IllegalInstruction(_s) => 2,
            Exception::Breakpoint => 3,
            Exception::LoadAddressMisaligned => 4,
            Exception::LoadAccessFault => 5,
            Exception::StoreAMOAddressMisaligned => 6,
            Exception::StoreAMOAccessFault => 7,
            Exception::EnvironmentCallFromUMode => 8,
            Exception::EnvironmentCallFromSMode => 9,
            Exception::EnvironmentCallFromMMode => 11,
            Exception::InstructionPageFault => 12,
            Exception::LoadPageFault => 13,
            Exception::StoreAMOPageFault => 15,
        }
    }
    /// Update CSRs and the program counter depending on an exception.
    pub fn take_trap(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        dbg!(format!("exception {:?}", self));
        let exception_pc = (cpu.pc as i64) - 4;

        let medeleg = cpu.state.read(MEDELEG);
        let sedeleg = cpu.state.read(SEDELEG);
        let pos = self.exception_code() & 0xffff;
        match ((medeleg >> pos) & 1) == 0 {
            true => cpu.mode = Mode::Machine,
            false => match ((sedeleg >> pos) & 1) == 0 {
                true => cpu.mode = Mode::Supervisor,
                false => cpu.mode = Mode::User,
            },
        }

        match cpu.mode {
            Mode::Machine => {
                // Set the program counter to the machine trap-handler base address (mtvec).
                cpu.pc = (cpu.state.read(MTVEC) & !1) as usize;

                cpu.state.write(MCAUSE, self.exception_code());
                cpu.state.write(MEPC, exception_pc);
                cpu.state.write(MTVAL, exception_pc);

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
                // Set the program counter to the supervisor trap-handler base address (stvec).
                cpu.pc = (cpu.state.read(STVEC) & !1) as usize;

                cpu.state.write(SCAUSE, self.exception_code());
                cpu.state.write(SEPC, exception_pc);
                cpu.state.write(STVAL, exception_pc);

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
                // Set the program counter to the user trap-handler base address (utvec).
                cpu.pc = (cpu.state.read(UTVEC) & !1) as usize;

                cpu.state.write(UCAUSE, self.exception_code());
                cpu.state.write(UEPC, exception_pc);

                // TODO: implement to update USTATUS
            }
            _ => {}
        }

        match self {
            Exception::InstructionAddressMisaligned => Err(Exception::InstructionAddressMisaligned),
            Exception::InstructionAccessFault => Err(Exception::InstructionAccessFault),
            Exception::IllegalInstruction(s) => Err(Exception::IllegalInstruction(s.to_string())),
            Exception::Breakpoint => Err(Exception::Breakpoint),
            Exception::LoadAddressMisaligned => Err(Exception::LoadAddressMisaligned),
            Exception::LoadAccessFault => Err(Exception::LoadAccessFault),
            Exception::StoreAMOAddressMisaligned => Err(Exception::StoreAMOAddressMisaligned),
            Exception::StoreAMOAccessFault => Err(Exception::StoreAMOAccessFault),
            Exception::EnvironmentCallFromUMode => Ok(()),
            Exception::EnvironmentCallFromSMode => Ok(()),
            Exception::EnvironmentCallFromMMode => Ok(()),
            Exception::InstructionPageFault => Err(Exception::InstructionPageFault),
            Exception::LoadPageFault => Err(Exception::LoadPageFault),
            Exception::StoreAMOPageFault => Err(Exception::StoreAMOPageFault),
        }
    }
}
