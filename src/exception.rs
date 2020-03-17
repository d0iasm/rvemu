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
                cpu.mode = Mode::Debug;
                // "ECALL and EBREAK cause the receiving privilege mode’s epc register to be set to
                // the address of the ECALL or EBREAK instruction itself, not the address of the
                // following instruction."
                cpu.state.write(MEPC, exception_pc);
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

                // Move to the more privileged mode.
                cpu.mode = Mode::Machine;
                // "ECALL and EBREAK cause the receiving privilege mode’s epc register to be set to
                // the address of the ECALL or EBREAK instruction itself, not the address of the
                // following instruction."
                cpu.state.write(MEPC, exception_pc);

                // Set the program counter to the machine trap-handler base address (mtvec) depending on the mode from mtvec.
                match cpu.state.read_bits(MTVEC, ..2) {
                    0 => {
                        // Direct mode.
                        let base = cpu.state.read_bits(MTVEC, 2..);
                        cpu.pc = base as usize;
                    }
                    1 => {
                        // Vectored mode.
                        let base = cpu.state.read_bits(MTVEC, 2..);
                        cpu.pc = (base + 4 * exception_code) as usize;
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction(String::from(
                            "illegal mode in mtvec",
                        )));
                    }
                }
            }
            Exception::EnvironmentCallFromSMode => {
                exception_code = 9;

                // Move to the more privileged mode.
                cpu.mode = Mode::Machine;
                // "ECALL and EBREAK cause the receiving privilege mode’s epc register to be set to
                // the address of the ECALL or EBREAK instruction itself, not the address of the
                // following instruction."
                cpu.state.write(MEPC, exception_pc);

                // Set the program counter to the machine trap-handler base address (mtvec) depending on the mode from mtvec.
                match cpu.state.read_bits(MTVEC, ..2) {
                    0 => {
                        // Direct mode.
                        let base = cpu.state.read_bits(MTVEC, 2..);
                        cpu.pc = base as usize;
                    }
                    1 => {
                        // Vectored mode.
                        let base = cpu.state.read_bits(MTVEC, 2..);
                        cpu.pc = (base + 4 * exception_code) as usize;
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction(String::from(
                            "illegal mode in mtvec",
                        )));
                    }
                }
            }
            Exception::EnvironmentCallFromMMode => {
                exception_code = 11;
                // Set the program counter to the machine trap-handler base address (mtvec) depending on the mode from mtvec.
                match cpu.state.read_bits(MTVEC, ..2) {
                    0 => {
                        // Direct mode.
                        let base = cpu.state.read_bits(MTVEC, 2..);
                        cpu.pc = base as usize;
                    }
                    1 => {
                        // Vectored mode.
                        let base = cpu.state.read_bits(MTVEC, 2..);
                        cpu.pc = (base + 4 * exception_code) as usize;
                    }
                    _ => {
                        return Err(Exception::IllegalInstruction(String::from(
                            "illegal mode in mtvec",
                        )));
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
        }

        match cpu.mode {
            Mode::Machine => {
                cpu.state.write(MCAUSE, 0 << 63 | exception_code);
                cpu.state.write(MEPC, exception_pc);
            }
            Mode::Supervisor => {
                cpu.state.write(SCAUSE, 0 << 63 | exception_code);
                cpu.state.write(SEPC, exception_pc);
            }
            Mode::User => {
                cpu.state.write(UCAUSE, 0 << 63 | exception_code);
                cpu.state.write(UEPC, exception_pc);
            }
            _ => {}
        }

        match self {
            Exception::InstructionAddressMisaligned(s) => {
                Err(Exception::InstructionAddressMisaligned(s.to_string()))
            }
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
