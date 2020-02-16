use crate::*;

#[derive(Debug)]
pub enum Exception {
    InstructionAddressMisaligned,
    IllegalInstruction,
}

impl Exception {
    pub fn take_trap(&self) {
        log(&format!("exception: {:#?}", self));
    }
}
