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

        let regs = &mut self.regs;

        match opcode {
            0x13 => {
                let imm = ((binary & 0xFFF00000) as i32) >> 20;
                match funct3 {
                    0x0 => regs[rd] = regs[rs1] + imm, // addi
                    0x2 => regs[rd] = if regs[rs1] < imm { 1 } else { 0 }, // slti
                    0x3 => regs[rd] = if (regs[rs1] as u32) < (imm as u32) { 1 } else { 0 }, // sltiu
                    0x4 => regs[rd] = regs[rs1] ^ imm, // xori
                    0x6 => regs[rd] = regs[rs1] | imm, // ori
                    0x7 => regs[rd] = regs[rs1] & imm, // andi
                    _ => {}
                }
            }
            0x33 => {
                match funct7 {
                    0x00 => regs[rd] = regs[rs1] + regs[rs2], // add rd, rs1, rs2
                    0x20 => regs[rd] = regs[rs1] - regs[rs2], // sub rd, rs1, rs2
                    _ => {},
                };
            }
            _ => {
                return;
            },
        }
    }
}
