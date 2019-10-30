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

// TODO: consider to divide into multiple `type` structs
/*
#[derive(Debug)]
struct RType {
    opcode: u8,
    funct3: u8,
    funct7: u8,
    rd: u8,
    rs1: u8,
    rs2: u8,
}

#[derive(Debug)]
struct IType {
    opcode: u8,
    funct3: u8,
    rd: u8,
    rs1: u8,
    imm: u32,
}

#[derive(Debug)]
enum Type {
    R_type(RType),
    I_type(IType),
    Unknown,
}

#[derive(Debug)]
pub struct Code {
    ty: Type
}
*/

#[derive(Debug)]
pub struct Code {
    opcode: u8,
    funct3: u8,
    funct7: u8,
    rd: u8,
    rs1: u8,
    rs2: u8,
    imm: u32,
}

impl Code {
    fn new(binary: u32) -> Code {
        let opcode: u8 = (binary & 0x0000007F).try_into().unwrap();

        match opcode {
            0x13 => { // I type
                let rd: u8 = ((binary & 0x00000F80).rotate_right(7))
                    .try_into().unwrap();
                let rs1 : u8 = ((binary & 0x000F8000).rotate_right(15))
                    .try_into().unwrap();
                let imm : u32 = (binary & 0xFFF00000).rotate_right(20);
                return Code {
                    opcode,
                    funct3: 0,
                    funct7: 0,
                    rd,
                    rs1,
                    rs2: 0,
                    imm,
                };
            }
            0x33 => { // R type
                let rd: u8 = ((binary & 0x00000F80).rotate_right(7))
                    .try_into().unwrap();
                let rs1 : u8 = ((binary & 0x000F8000).rotate_right(15))
                    .try_into().unwrap();
                let rs2 : u8 = ((binary & 0x01F00000).rotate_right(20))
                    .try_into().unwrap();
                return Code {
                    opcode,
                    funct3: 0,
                    funct7: 0,
                    rd,
                    rs1,
                    rs2,
                    imm: 0,
                };
            }
            _ => {}
        }

        Code {
            opcode: opcode,
            funct3: 0,
            funct7: 0,
            rd: 0,
            rs1: 0,
            rs2: 0,
            imm: 0,
        }
    }
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
        let mut bin: u32 = 0;

        // little endian
        for i in 0..4 {
            let v: u32 = memory[self.pc + i].into();
            bin = bin.rotate_left(8) + v;
        }
        self.pc += 4;
        return bin;
    }

    pub fn decode(&self, binary: u32) -> Code {
        Code::new(binary)
    }

    pub fn execute(&mut self, code: &Code, memory: &mut Vec<u8>) {
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
