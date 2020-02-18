use std::process::exit;

use crate::*;

#[derive(Debug)]
pub enum Exception {
    InstructionAddressMisaligned(String),
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
    pub fn take_trap(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let exception_code;
        match self {
            Exception::InstructionAddressMisaligned(_s) => {
                exception_code = 0;
            }
            Exception::InstructionAccessFault => {
                exception_code = 1;
            }
            Exception::IllegalInstruction(_s) => {
                exception_code = 2;
            }
            Exception::Breakpoint => {
                exception_code = 3;
            }
            Exception::LoadAddressMisaligned => {
                exception_code = 4;
            }
            Exception::LoadAccessFault => {
                exception_code = 5;
            }
            Exception::StoreAMOAddressMisaligned => {
                exception_code = 6;
            }
            Exception::StoreAMOAccessFault => {
                exception_code = 7;
            }
            Exception::EnvironmentCallFromUMode => {
                exception_code = 8;
            }
            Exception::EnvironmentCallFromSMode => {
                exception_code = 9;
            }
            Exception::EnvironmentCallFromMMode => {
                exception_code = 11;
            }
            Exception::InstructionPageFault => {
                exception_code = 12;
            }
            Exception::LoadPageFault => {
                exception_code = 13;
            }
            Exception::StoreAMOPageFault => {
                exception_code = 15;
            }
        }

        match cpu.mode {
            Mode::Machine => cpu.csr.write(MCAUSE_ADDRESS, 0 << 63 | exception_code)?,
            Mode::Supervisor => cpu.csr.write(SCAUSE_ADDRESS, 0 << 63 | exception_code)?,
            Mode::User => cpu.csr.write(UCAUSE_ADDRESS, 0 << 63 | exception_code)?,
            _ => {}
        }

        output(&format!("exception: {:#?}", self));
        output(&format!("mcause: {:#?}", cpu.csr.read(MCAUSE_ADDRESS)));
        exit(1);
    }
}