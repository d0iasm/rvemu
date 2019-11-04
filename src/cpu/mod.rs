use crate::*;

pub const REGISTERS_COUNT: usize = 32;

pub struct Cpu {
    pub registers: [u32; REGISTERS_COUNT],
    pub pc: usize,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: [0; REGISTERS_COUNT],
            pc: 0,
        }
    }

    pub fn start(&mut self, memory: &mut Vec<u8>) {
        let size = memory.len();

        while self.pc < size {
            let binary = self.fetch(memory);
            self.execute(binary, memory);
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

    fn execute(&mut self, binary: u32, memory: &mut Vec<u8>) {
        let opcode = binary & 0x0000007F;
        let rd = ((binary & 0x00000F80) >> 7) as usize;
        let rs1 = ((binary & 0x000F8000) >> 15) as usize;
        let rs2 = ((binary & 0x01F00000) >> 20) as usize;
        let funct3 = (binary & 0x00007000) >> 12;
        let funct7 = (binary & 0xFE000000) >> 25;
        let imm = (binary & 0xFFF00000) >> 20;

        let text = format!("pc = {:x}, opcode = {:#x} ({}, {:#b})",
            self.pc, opcode, opcode, opcode);
        log(&text);
        render(&text);

        match opcode {
            0x13 => {
                // addi rd, rs1, imm (0x13): I-type. rd = rs1 + imm
                self.registers[rd] = self.registers[rs1] + imm;
            },
            0x33 => {
                // add rd, rs1, rs2 (0x33): R-type. rd = rs1 + rs2
                self.registers[rd] = self.registers[rs1] + self.registers[rs2];
            },
            _ => {
                let text = format!("not implemented: opecode {:#x} ({}, {:#b})",
                    opcode, opcode, opcode);
                log(&text);
                render(&text);
                return;
            },
        }
    }
}
