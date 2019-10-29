pub mod register;
pub mod instruction;

use crate::log;
use crate::cpu::register::REGISTERS_COUNT;
use crate::cpu::instruction::*;

pub struct Cpu {
    pub instructions: [Option<InstFunc>; 127],
    pub registers: [u32; REGISTERS_COUNT],
    pub pc: usize,
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut insts = [None; 127];
        init_instructions(&mut insts);

        Cpu {
            instructions: insts,
            registers: [0; REGISTERS_COUNT],
            pc: 0,
        }
    }

    pub fn fetch(&mut self, memory: &Vec<u8>) -> u32 {
        let mut ret: u32 = 0;

        // little endian
        for i in 0..4 {
            let v: u32 = memory[self.pc + i].into();
            ret = (ret << (i * 8)) | v;
        }
        self.pc += 4;
        return ret;
    }

    pub fn decode(&self) {}

    pub fn execute(&mut self, code: u32, memory: &mut Vec<u8>) {
        // TODO: how to get opcode?
        let opcode: usize = (code & 0x0000007F) as usize;

        log(&format!("pc = {}, opcode = {}, code = {}", self.pc, opcode, code));

        match self.instructions[opcode] {
            Some(inst) => inst(self, memory),
            None => log(&format!("not implemented: opecode {}, {}", opcode, code)),
        }
    }
}
