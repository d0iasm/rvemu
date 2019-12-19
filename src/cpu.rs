pub const REGISTERS_COUNT: usize = 32;

use std::process::exit;

use num_bigint::{BigUint, BigInt};
use num_traits::cast::ToPrimitive;

use crate::*;

pub struct Cpu {
    pub regs: [i64; REGISTERS_COUNT],
    pub pc: usize,
}

fn set_memory8(index: usize, mem: &mut Vec<u8>, val: u8) {
    mem[index] = val
}

fn set_memory16(index: usize, mem: &mut Vec<u8>, val: u16) {
    mem[index] = (val & 0xFF) as u8;
    mem[index + 1] = ((val & 0xFF00) >> 8) as u8;
}

fn set_memory32(index: usize, mem: &mut Vec<u8>, val: u32) {
    mem[index] = (val & 0xFF) as u8;
    mem[index + 1] = ((val & 0xFF00) >> 8) as u8;
    mem[index + 2] = ((val & 0xFF0000) >> 16) as u8;
    mem[index + 3] = ((val & 0xFF000000) >> 24) as u8;
}

fn set_memory64(index: usize, mem: &mut Vec<u8>, val: u64) {
    mem[index] = (val & 0xFF) as u8;
    mem[index + 1] = ((val & 0xFF00) >> 8) as u8;
    mem[index + 2] = ((val & 0xFF0000) >> 16) as u8;
    mem[index + 3] = ((val & 0xFF000000) >> 24) as u8;
    mem[index + 4] = ((val & 0xFF00000000) >> 32) as u8;
    mem[index + 5] = ((val & 0xFF0000000000) >> 40) as u8;
    mem[index + 6] = ((val & 0xFF000000000000) >> 48) as u8;
    mem[index + 7] = ((val & 0xFF00000000000000) >> 56) as u8;
}

fn get_memory8(index: usize, mem: &Vec<u8>) -> u8 {
    mem[index]
}

fn get_memory16(index: usize, mem: &Vec<u8>) -> u16 {
    // little endian
    return (mem[index] as u16)
        | ((mem[index + 1] as u16) << 8);
}

fn get_memory32(index: usize, mem: &Vec<u8>) -> u32 {
    // little endian
    return (mem[index] as u32)
        | ((mem[index + 1] as u32) << 8)
        | ((mem[index + 2] as u32) << 16)
        | ((mem[index + 3] as u32) << 24);
}

fn get_memory64(index: usize, mem: &Vec<u8>) -> u64 {
    // little endian
    return (mem[index] as u64)
        | ((mem[index + 1] as u64) << 8)
        | ((mem[index + 2] as u64) << 16)
        | ((mem[index + 3] as u64) << 24)
        | ((mem[index + 4] as u64) << 32)
        | ((mem[index + 5] as u64) << 40)
        | ((mem[index + 6] as u64) << 48)
        | ((mem[index + 7] as u64) << 56);
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            regs: [0; REGISTERS_COUNT],
            pc: 0,
        }
    }

    pub fn start(&mut self, mem: &mut Vec<u8>) {
        let size = mem.len();

        let mut i = 0;
        while self.pc < size {
            let binary = self.fetch(mem);
            self.execute(binary, mem);
            self.pc += 4;

            // TODO: Remove the following check.
            // This is for avoiding an infinite execution.
            i += 1;
            if i > 1000 {
                exit(1);
            }
        }
    }

    fn fetch(&mut self, mem: &Vec<u8>) -> u32 {
        get_memory32(self.pc, mem)
    }

    // This function is public because it's called from a unit test.
    pub fn execute(&mut self, binary: u32, mem: &mut Vec<u8>) {
        let opcode = binary & 0x0000007F;
        let rd = ((binary & 0x00000F80) >> 7) as usize;
        let rs1 = ((binary & 0x000F8000) >> 15) as usize;
        let rs2 = ((binary & 0x01F00000) >> 20) as usize;
        let funct3 = (binary & 0x00007000) >> 12;
        let funct7 = (binary & 0xFE000000) >> 25;

        let regs = &mut self.regs;

        log(&format!("execute pc: {} ({:#x}), opcode: {} ({:#x}, {:#b}), binary: {:#x}",
                    self.pc, self.pc, opcode, opcode, opcode, binary));
        match opcode {
            0x03 => { // I-type
                let offset = (((binary & 0xFFF00000) as i32) as i64) >> 20;
                let addr = (regs[rs1] + offset) as usize;
                match funct3 {
                    0x0 => regs[rd] = (get_memory8(addr, mem) as i8) as i64, // lb
                    0x1 => regs[rd] = (get_memory16(addr, mem) as i16) as i64, // lh
                    0x2 => regs[rd] = (get_memory32(addr, mem) as i32) as i64, // lw
                    0x3 => regs[rd] = get_memory64(addr, mem) as i64, // ld
                    0x4 => regs[rd] = (get_memory8(addr, mem) as i64) & 0xFF, // lbu
                    0x5 => regs[rd] = (get_memory16(addr, mem) as i64) & 0xFFFF, // lhu
                    0x6 => regs[rd] = (get_memory32(addr, mem) as i64) & 0xFFFFFFFF, // lwu
                    _ => {},
                }
            },
            0x0F => { // I-type
                // fence instructions are not supportted yet because this emulator executes a
                // binary sequentially on a single thread.
                // fence i is a part of the Zifencei extension.
                match funct3 {
                    0x0 => {}, // fence
                    0x1 => {}, // fence.i
                    _ => {},
                }
            }
            0x13 => { // I-type
                let imm = (((binary & 0xFFF00000) as i32) as i64) >> 20;
                // shamt size is 5 bits for RV32I and 6 bits for RV64I.
                // let shamt = (binary & 0x01F00000) >> 20; // This is for RV32I
                let shamt = (binary & 0x03F00000) >> 20;
                let funct6 = funct7 >> 1;
                match funct3 {
                    0x0 => regs[rd] = regs[rs1].wrapping_add(imm), // addi
                    0x1 => regs[rd] = ((regs[rs1] as u64) << shamt) as i64, // slli
                    0x2 => regs[rd] = if regs[rs1] < imm { 1 } else { 0 }, // slti
                    0x3 => regs[rd] = if (regs[rs1] as u64) < (imm as u64) { 1 } else { 0 }, // sltiu
                    0x4 => regs[rd] = regs[rs1] ^ imm, // xori
                    0x5 => {
                        match funct6 {
                            0x00 => regs[rd] = ((regs[rs1] as u64) >> shamt) as i64, // srli
                            0x10 => regs[rd] = regs[rs1] >> shamt, // srai
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
                let imm = ((binary & 0xFFFFF000) as i32) as i64;
                regs[rd] = (self.pc as i64) + imm; // auipc
            },
            0x1B => { // I-type (RV64I only)
                let imm = (((binary & 0xFFF00000) as i32) as i64) >> 20;
                let shamt = (binary & 0x01F00000) >> 20;
                match funct3 {
                    0x0 => regs[rd] = (((regs[rs1].wrapping_add(imm)) & 0xFFFFFFFF) as i32) as i64, // addiw
                    0x1 => regs[rd] = (((regs[rs1] << shamt) & 0xFFFFFFFF) as i32) as i64, // slliw
                    0x5 => {
                        match funct7 {
                            0x00 => regs[rd] = ((regs[rs1] as u32) >> shamt) as i64, // srliw
                            0x20 => regs[rd] = ((regs[rs1] as i32) >> shamt) as i64, // sraiw
                            _ => {},
                        }
                    }
                    _ => {},
                }
            }
            0x23 => { // S-type
                let imm11_5 = (((binary & 0xFE000000) as i32) as i64) >> 25;
                let imm4_0 = ((binary & 0x00000F80) >> 7) as u64;
                let offset = (((imm11_5 << 5) as u64) | imm4_0) as i64;
                let addr = (regs[rs1] + offset) as usize;
                match funct3 {
                    0x0 => set_memory8(addr, mem, regs[rs2] as u8), // sb
                    0x1 => set_memory16(addr, mem, regs[rs2] as u16), // sh
                    0x2 => set_memory32(addr, mem, regs[rs2] as u32), // sw
                    0x3 => set_memory64(addr, mem, regs[rs2] as u64), // sd
                    _ => {},
                }
            },
            0x33 => { // R-type (RV32I and RV32M)
                let shamt = regs[rs2] as u64;
                match (funct3, funct7) {
                    (0x0, 0x00) => regs[rd] = regs[rs1].wrapping_add(regs[rs2]), // add
                    (0x0, 0x01) => regs[rd] = regs[rs1].wrapping_mul(regs[rs2]), // mul
                    (0x0, 0x20) => regs[rd] = regs[rs1].wrapping_sub(regs[rs2]), // sub
                    (0x1, 0x00) => regs[rd] = ((regs[rs1] as u64) << shamt) as i64, // sll
                    (0x1, 0x01) => { // mulh
                        let n1 = BigInt::from(regs[rs1]);
                        let n2 = BigInt::from(regs[rs2]);
                        regs[rd] = ((n1 * n2) >> 64).to_i64().unwrap();
                    },
                    (0x2, 0x00) => regs[rd] = if regs[rs1] < regs[rs2] { 1 } else { 0 }, // slt
                    (0x2, 0x01) => { // mulhsu
                        // get the most significant bit
                        let sign = ((regs[rs1] as u64) & 0x80000000_00000000) as i64;
                        // regs[rs1] is signed and regs[rs2] is unsigned
                        let n1 = BigUint::from((regs[rs1] as u64) & 0xefffffff_ffffffff);
                        let n2 = BigUint::from(regs[rs2] as u64);
                        regs[rd] = sign | ((n1 * n2) >> 64).to_i64().unwrap();
                    },
                    (0x3, 0x00) => regs[rd] = if (regs[rs1] as u64) < (regs[rs2] as u64) { 1 } else { 0 }, // sltu
                    (0x3, 0x01) => { // mulhu
                        let n1 = BigUint::from(regs[rs1] as u64);
                        let n2 = BigUint::from(regs[rs2] as u64);
                        regs[rd] = ((n1 * n2) >> 64).to_i64().unwrap();
                    },
                    (0x4, 0x00) => regs[rd] = regs[rs1] ^ regs[rs2], // xor
                    (0x4, 0x01) => regs[rd] = regs[rs1].wrapping_div(regs[rs2]), // div
                    (0x5, 0x00) => regs[rd] = ((regs[rs1] as u64) >> shamt) as i64, // srl
                    (0x5, 0x01) => { // divu
                        let dividend = regs[rs1] as u64;
                        let divisor = regs[rs2] as u64;
                        regs[rd] = dividend.wrapping_div(divisor) as i64;
                    },
                    (0x5, 0x20) => regs[rd] = regs[rs1] >> shamt, // sra
                    (0x6, 0x00) => regs[rd] = regs[rs1] | regs[rs2], // or
                    (0x6, 0x01) => regs[rd] = regs[rs1] % regs[rs2], // rem
                    (0x7, 0x00) => regs[rd] = regs[rs1] & regs[rs2], // and
                    (0x7, 0x01) => { // remu
                        let dividend = regs[rs1] as u64;
                        let divisor = regs[rs2] as u64;
                        regs[rd] = (dividend % divisor) as i64;
                    },
                    _ => {},
                };
            },
            0x37 => { // U-type
                // LUI places the U-immediate value in the top 20 bits of the destination
                // register rd, filling in the lowest 12 bits with zeros.
                regs[rd] = ((binary & 0xFFFFF000) as i32) as i64; // lui
            },
            0x3B => { // R-type (RV64I and RV64M)
                // The shift amount is given by rs2[4:0].
                let shamt = (regs[rs2] & 0x1F) as u32;
                match (funct3, funct7) {
                    (0x0, 0x00) => regs[rd] = ((regs[rs1].wrapping_add(regs[rs2])) as i32) as i64, // addw
                    (0x0, 0x01) => { // mulw
                        let n1 = regs[rs1] as i32;
                        let n2 = regs[rs2] as i32;
                        let result = n1.wrapping_mul(n2);
                        regs[rd] = result as i64;
                    },
                    (0x0, 0x20) => regs[rd] = ((regs[rs1].wrapping_sub(regs[rs2])) as i32) as i64, // subw
                    (0x1, 0x00) => regs[rd] = (((regs[rs1] as u32) << shamt) as i32) as i64, // sllw
                    (0x4, 0x01) => { // divw
                        let dividend = regs[rs1] as i32;
                        let divisor = regs[rs2] as i32;
                        regs[rd] = dividend.wrapping_div(divisor) as i64;
                    },
                    (0x5, 0x00) => regs[rd] = (((regs[rs1] as u32) >> shamt) as i32) as i64, // srlw
                    (0x5, 0x01) => { // divuw
                        let dividend = regs[rs1] as u32;
                        let divisor = regs[rs2] as u32;
                        regs[rd] = (dividend.wrapping_div(divisor) as i32) as i64;
                    },
                    (0x5, 0x20) => regs[rd] = ((regs[rs1] as i32) >> (shamt as i32)) as i64, // sraw
                    (0x6, 0x01) => { // remw
                        let dividend = regs[rs1] as i32;
                        let divisor = regs[rs2] as i32;
                        regs[rd] = dividend.wrapping_rem(divisor) as i64;
                    },
                    (0x7, 0x01) => { // remuw
                        let dividend = regs[rs1] as u32;
                        let divisor = regs[rs2] as u32;
                        regs[rd] = (dividend.wrapping_rem(divisor) as i32) as i64;
                    },
                    _ => {},
                }
            }
            0x63 => { // B-type
                let imm12 = (((binary & 0x80000000) as i32) as i64) >> 31;
                let imm10_5 = ((binary & 0x7E000000) >> 25) as u64;
                let imm4_1 = ((binary & 0x00000F00) >> 8) as u64;
                let imm11 = ((binary & 0x00000080) >> 7) as u64;
                let offset = ((imm12 << 12) as u64
                    | (imm11 << 11)
                    | (imm10_5 << 5)
                    | (imm4_1 << 1)) as i64;
                match funct3 {
                    0x0 => {
                        // beq
                        if regs[rs1] == regs[rs2] {
                            self.pc = ((self.pc as i64) + offset) as usize;
                        }
                    },
                    0x1 => {
                        // bne
                        if regs[rs1] != regs[rs2] {
                            self.pc = ((self.pc as i64) + offset) as usize;
                        }
                    },
                    0x4 => {
                        // blt
                        if regs[rs1] < regs[rs2] {
                            self.pc = ((self.pc as i64) + offset) as usize;
                        }
                    },
                    0x5 => {
                        // bge
                        if regs[rs1] >= regs[rs2] {
                            self.pc = ((self.pc as i64) + offset) as usize;
                            // TODO: Check if this operation is valid
                            self.pc -= 4;
                        }
                    },
                    0x6 => {
                        // bltu
                        if (regs[rs1] as u64) < (regs[rs2] as u64) {
                            self.pc = ((self.pc as i64) + offset) as usize;
                        }
                    },
                    0x7 => {
                        // bgeu
                        if (regs[rs1] as u64) >= (regs[rs2] as u64) {
                            self.pc = ((self.pc as i64) + offset) as usize;
                        }
                    },
                    _ => {},
                }
            },
            0x67 => { // I-type
                // jalr
                regs[rd] = (self.pc as i64) + 4;

                let imm = (((binary & 0xFFF00000) as i32) as i64) >> 20;
                self.pc = ((regs[rs1] + imm) & !1) as usize;
            },
            0x6F => { // J-type
                // jal
                regs[rd] = (self.pc as i64) + 4;

                let imm20 = (((binary & 0x80000000) as i32) as i64) >> 31;
                let imm10_1 = ((binary & 0x7FE00000) >> 21) as u64;
                let imm11 = ((binary & 0x100000) >> 20) as u64;
                let imm19_12 = ((binary & 0xFF000) >> 12) as u64;
                let offset = ((imm20 << 20) as u64
                    | (imm19_12 << 12)
                    | (imm11 << 11)
                    | (imm10_1 << 1)) as i64;
                let tmp = (self.pc as i64) + offset;
                self.pc = tmp as usize;
                // TODO: Check if this operation is valid
                self.pc -= 4;
            },
            0x73 => { // I-type
                let funct12 = (((binary & 0xFFF00000) as i32) as i64) >> 20;
                let _csr = (((binary & 0xFFF00000) as i32) as i64) >> 20;
                match funct3 {
                    0x0 => {
                        match funct12 {
                            // TODO: implement ecall and ebreak
                            // ecall makes a request of the execution environment by raising an
                            // environment call exception.
                            // ebreak makes a request of the debugger by raising a breakpoint
                            // exception.
                            0x0 => {}, // ecall
                            0x1 => {}, // ebreak
                            _ => {},
                        }
                    },
                    // TODO: implement RV32/RV64 Zicsr Standard Extension
                    0x1 => {}, // csrrw
                    0x2 => {}, // csrrs
                    0x3 => {}, // csrrc
                    0x5 => {}, // csrrwi
                    0x6 => {}, // csrrsi
                    0x7 => {}, // csrrci
                    _ => {},
                }
            },
            _ => {
                let text = format!("not implemented opcode {} ({:#x}, {:#b})",
                                   opcode, opcode, opcode);
                log(&text);
                render(&text);
                exit(1);
            },
        }
    }
}
