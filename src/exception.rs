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
        let exception_pc = (cpu.pc as i64) - 4;

        match self {
            Exception::Breakpoint => {
                cpu.mode = Mode::Debug;
            }
            Exception::EnvironmentCallFromUMode => {
                // Move to the more privileged mode.
                cpu.mode = Mode::Machine;
            }
            Exception::EnvironmentCallFromSMode => {
                // Move to the more privileged mode.
                cpu.mode = Mode::Machine;
            }
            _ => {}
        }

        match cpu.mode {
            Mode::Machine => {
                // Set the program counter to the machine trap-handler base address (mtvec).
                cpu.pc = cpu.state.read_bits(MTVEC, 2..) as usize;

                cpu.state.write(MCAUSE, self.exception_code());
                cpu.state.write(MEPC, exception_pc);
            }
            Mode::Supervisor => {
                // Set the program counter to the supervisor trap-handler base address (stvec).
                cpu.pc = cpu.state.read_bits(STVEC, 2..) as usize;

                cpu.state.write(SCAUSE, self.exception_code());
                cpu.state.write(SEPC, exception_pc);
            }
            Mode::User => {
                cpu.state.write(UCAUSE, self.exception_code());
                cpu.state.write(UEPC, exception_pc);
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
