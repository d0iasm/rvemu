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
            self.pc += 4;
        }
    }

    fn fetch(&mut self, memory: &Vec<u8>) -> u32 {
        // little endian
        return ((memory[self.pc] as u32) << 24)
            + ((memory[self.pc + 1] as u32) << 16)
            + ((memory[self.pc + 2] as u32) << 8)
            + (memory[self.pc + 3] as u32);
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
            0x13 => { // I-type
                let imm = ((binary & 0xFFF00000) as i32) >> 20;
                let shamt = (binary & 0x01F00000) >> 20;
                match funct3 {
                    0x0 => regs[rd] = regs[rs1] + imm, // addi
                    0x1 => regs[rd] = ((regs[rs1] as u32) << shamt) as i32, // slli
                    0x2 => regs[rd] = if regs[rs1] < imm { 1 } else { 0 }, // slti
                    0x3 => regs[rd] = if (regs[rs1] as u32) < (imm as u32) { 1 } else { 0 }, // sltiu
                    0x4 => regs[rd] = regs[rs1] ^ imm, // xori
                    0x5 => {
                        match funct7 {
                            0x00 => regs[rd] = ((regs[rs1] as u32) >> shamt) as i32, // srli
                            0x20 => regs[rd] = regs[rs1] >> shamt, // srai
                            _ => {},
                        }
                    }
                    0x6 => regs[rd] = regs[rs1] | imm, // ori
                    0x7 => regs[rd] = regs[rs1] & imm, // andi
                    _ => {},
                }
            },
            0x17 => { // U-type
                // AUIPC forms a 32-bit offset from the 20-bit U-immediate, filling
                // in the lowest 12 bits with zeros.
                let imm = (binary & 0xFFFFF000) as i32;
                regs[rd] = (self.pc as i32) + imm; // auipc
            }
            0x33 => { // R-type
                match (funct3, funct7) {
                    (0x0, 0x00) => regs[rd] = regs[rs1] + regs[rs2], // add
                    (0x0, 0x20) => regs[rd] = regs[rs1] - regs[rs2], // sub
                    (0x1, 0x00) => regs[rd] = ((regs[rs1] as u32) << (regs[rs2] as u32)) as i32, // sll
                    (0x2, 0x00) => regs[rd] = if regs[rs1] < regs[rs2] { 1 } else { 0 }, // slt
                    (0x3, 0x00) => regs[rd] = if (regs[rs1] as u32) < (regs[rs2] as u32) { 1 } else { 0 }, // sltu
                    (0x4, 0x00) => regs[rd] = regs[rs1] ^ regs[rs2], // xor
                    (0x5, 0x00) => regs[rd] = ((regs[rs1] as u32) >> (regs[rs2] as u32)) as i32, // srl
                    (0x5, 0x20) => regs[rd] = regs[rs1] >> (regs[rs2] as u32), // sra
                    (0x6, 0x00) => regs[rd] = regs[rs1] | regs[rs2], // or
                    (0x7, 0x00) => regs[rd] = regs[rs1] & regs[rs2], // and
                    _ => {},
                };
            },
            0x37 => { // U-type
                // LUI places the U-immediate value in the top 20 bits of the destination
                // register rd, filling in the lowest 12 bits with zeros.
                regs[rd] = (binary & 0xFFFFF000) as i32; // lui
            },
            0x67 => { // I-type
                // jalr
                regs[rd] = (self.pc as i32) + 4;

                let imm = ((binary & 0xFFF00000) as i32) >> 20;
                self.pc = ((regs[rs1] + imm) & !1) as usize;
            },
            0x6F => { // J-type
                // jal
                regs[rd] = (self.pc as i32) + 4;

                let imm20 = ((binary & 0x80000000) as i32) >> 31;
                let imm10_1 = (binary & 0x7FE00000) >> 21;
                let imm11 = (binary & 0x100000) >> 20;
                let imm19_12 = (binary & 0xFF000) >> 12;
                let offset = ((imm20 << 20) as u32
                    | (imm19_12 << 12)
                    | (imm11 << 11)
                    | (imm10_1) << 1) as i32;
                let tmp = (self.pc as i32) + offset;
                self.pc = tmp as usize;
            },
            _ => {},
        }
    }
}
