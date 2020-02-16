use std::process::exit;

use crate::*;

#[derive(Debug)]
pub enum Exception {
    InstructionAddressMisaligned(String),
    IllegalInstruction(String),
    EnvironmentCallFromUMode,
    EnvironmentCallFromSMode,
    EnvironmentCallFromMMode,
    Breakpoint,
}

impl Exception {
    pub fn take_trap(&self) {
        match self {
            Exception::InstructionAddressMisaligned(_s) => {}
            Exception::IllegalInstruction(_s) => {}
            Exception::EnvironmentCallFromUMode => {}
            Exception::EnvironmentCallFromSMode => {}
            Exception::EnvironmentCallFromMMode => {}
            Exception::Breakpoint => {}
        }
        output(&format!("exception: {:#?}", self));
        exit(1);
    }
}
