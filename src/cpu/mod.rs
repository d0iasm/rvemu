pub mod register;
pub mod instruction;

use crate::*;
use crate::cpu::register::*;
use crate::cpu::instruction::*;

use std::convert::TryInto;

pub struct Cpu {
    pub instructions: [Option<InstFunc>; 127],
    pub registers: [u32; REGISTERS_COUNT],
    pub pc: usize,
}

#[derive(Debug)]
pub struct Code {
    opcode: u32,
    funct3: u32,
    funct7: u32,
    rd: u32,
    rs1: u32,
    rs2: u32,
    imm: u32,
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

    pub fn start(&mut self, memory: &mut Vec<u8>) {
        let size = memory.len();

        while self.pc < size {
            let binary = self.fetch(memory);
            let code = self.decode(binary);
            self.execute(&code, memory);
        }
    }

    fn fetch(&mut self, memory: &Vec<u8>) -> u32 {
        let mut bin: u32 = 0;

        // little endian
        for i in 0..4 {
            let v: u32 = memory[self.pc + i].into();
            bin = bin.rotate_left(8) + v;
        }
        self.pc += 4;
        return bin;
    }

    fn decode(&self, binary: u32) -> Code {
        let opcode = binary & 0x0000007F;
        let rd = (binary & 0x00000F80) >> 7;
        let funct3 = (binary & 0x00007000) >> 12;
        let rs1 = (binary & 0x000F8000) >> 15;
        let rs2 = (binary & 0x01F00000) >> 20;
        let imm = (binary & 0xFFF00000) >> 20;
        let funct7 = (binary & 0xFE000000) >> 25;
        Code{ opcode, funct3, funct7, rd, rs1, rs2, imm }
    }

    fn execute(&mut self, code: &Code, memory: &mut Vec<u8>) {
        let opcode: usize = code.opcode as usize;

        let text = format!("pc = {:x}, opcode = {:#x} ({}, {:#b}), code = {:#?})",
            self.pc, opcode, opcode, opcode, code);
        log(&text);
        render(&text);

        match self.instructions[opcode] {
            Some(inst) => inst(self, &code, memory),
            None => {
                let text = format!("not implemented: opecode {}, {:#?}", opcode, code);
                log(&text);
                render(&text);
                return;
            },
        }
    }
}
