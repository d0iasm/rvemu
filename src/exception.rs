//! The exception module contains all the exception kinds and the function to handle exceptions.

use crate::{
    cpu::{Cpu, Mode},
    csr::*,
};

/// All the exception kinds.
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
    Unimplemented,
}

impl Exception {
    /// Update CSRs and the program counter depending on an exception.
    pub fn take_trap(&self, cpu: &mut Cpu) -> Result<(), Exception> {
        let exception_code;
        let exception_pc = (cpu.pc as i64) - 4;

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
                match cpu.state.get(MTVEC)? {
                    Csr::Mtvec(mtvec) => match mtvec.read_mode() {
                        mtvec::Mode::Direct => {
                            cpu.pc = mtvec.read_base() as usize;
                        }
                        mtvec::Mode::Vectored => {
                            cpu.pc = (mtvec.read_base() + 4 * exception_code) as usize;
                        }
                        _ => {
                            return Err(Exception::IllegalInstruction(String::from(
                                "illegal mode in mtvec",
                            )))
                        }
                    },
                    _ => {
                        return Err(Exception::IllegalInstruction(String::from(
                            "failed to get a mtvec csr",
                        )))
                    }
                }
            }
            Exception::EnvironmentCallFromSMode => {
                exception_code = 9;
                match cpu.state.get(MTVEC)? {
                    Csr::Mtvec(mtvec) => match mtvec.read_mode() {
                        mtvec::Mode::Direct => {
                            cpu.pc = mtvec.read_base() as usize;
                        }
                        mtvec::Mode::Vectored => {
                            cpu.pc = (mtvec.read_base() + 4 * exception_code) as usize;
                        }
                        _ => {
                            return Err(Exception::IllegalInstruction(String::from(
                                "illegal mode in mtvec",
                            )))
                        }
                    },
                    _ => {
                        return Err(Exception::IllegalInstruction(String::from(
                            "failed to get a mtvec csr",
                        )))
                    }
                }
            }
            Exception::EnvironmentCallFromMMode => {
                exception_code = 11;
                match cpu.state.get(MTVEC)? {
                    Csr::Mtvec(mtvec) => match mtvec.read_mode() {
                        mtvec::Mode::Direct => {
                            cpu.pc = mtvec.read_base() as usize;
                        }
                        mtvec::Mode::Vectored => {
                            cpu.pc = (mtvec.read_base() + 4 * exception_code) as usize;
                        }
                        _ => {
                            return Err(Exception::IllegalInstruction(String::from(
                                "illegal mode in mtvec",
                            )))
                        }
                    },
                    _ => {
                        return Err(Exception::IllegalInstruction(String::from(
                            "failed to get a mtvec csr",
                        )))
                    }
                }
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
            Exception::Unimplemented => {
                return Err(Exception::Unimplemented);
            }
        }

        match cpu.mode {
            Mode::Machine => {
                cpu.state.write(MCAUSE, 0 << 63 | exception_code)?;
                cpu.state.write(MEPC, exception_pc)?;
            }
            Mode::Supervisor => {
                cpu.state.write(SCAUSE, 0 << 63 | exception_code)?;
                cpu.state.write(SEPC, exception_pc)?;
            }
            Mode::User => {
                cpu.state.write(UCAUSE, 0 << 63 | exception_code)?;
                cpu.state.write(UEPC, exception_pc)?;
            }
            _ => {}
        }
        Ok(())
    }
}
