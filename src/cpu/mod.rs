use crate::*;

pub const REGISTERS_COUNT: usize = 32;

pub struct Cpu {
    pub regs: [i32; REGISTERS_COUNT],
    pub pc: usize,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            regs: [0; REGISTERS_COUNT],
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
        // little endian
        let bin = ((memory[self.pc] as u32) << 24)
            + ((memory[self.pc + 1] as u32) << 16)
            + ((memory[self.pc + 2] as u32) << 8)
            + (memory[self.pc + 3] as u32);
        self.pc += 4;
        return bin;
    }

    pub fn execute(&mut self, binary: u32, memory: &mut Vec<u8>) {
        let opcode = binary & 0x0000007F;
        let rd = ((binary & 0x00000F80) >> 7) as usize;
        let rs1 = ((binary & 0x000F8000) >> 15) as usize;
        let rs2 = ((binary & 0x01F00000) >> 20) as usize;
        let funct3 = (binary & 0x00007000) >> 12;
        let funct7 = (binary & 0xFE000000) >> 25;

        match opcode {
            0x13 => {
                // addi rd, rs1, imm
                let imm = ((binary & 0xFFF00000) as i32) >> 20;
                self.regs[rd] = self.regs[rs1] + imm;
            }
            0x33 => {
                match funct7 {
                    0x00 => self.regs[rd] = self.regs[rs1] + self.regs[rs2], // add rd, rs1, rs2
                    0x20 => self.regs[rd] = self.regs[rs1] - self.regs[rs2], // sub rd, rs1, rs2
                    _ => {},
                };
            }
            _ => {
                return;
            },
        }
    }
}
