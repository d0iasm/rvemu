pub const REGISTERS_COUNT: usize = 32;

use std::process::exit;

use num_bigint::{BigInt, BigUint};
use num_traits::cast::ToPrimitive;

use crate::*;

pub struct Cpu {
    pub xregs: [i64; REGISTERS_COUNT],
    pub fregs: [f32; REGISTERS_COUNT],
    pub pc: usize,
    /*
     *  31       8 7                   5 4                           0
     * | Reserved | Rounding Mode (frm) |  Accrued Exceptions    (fflags)  |
     *                                  |  NV  |  DZ  |  OF  |  UF  |  NX  |
     *      24              3              1      1      1      1      1
     */
    pub fcsr: u32,
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
    return (mem[index] as u16) | ((mem[index + 1] as u16) << 8);
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
            xregs: [0; REGISTERS_COUNT],
            fregs: [0.0; REGISTERS_COUNT],
            pc: 0,
            fcsr: 0,
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.fcsr = 0;
        for i in 0..REGISTERS_COUNT {
            self.xregs[i] = 0;
            self.fregs[i] = 0.0;
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

        let xregs = &mut self.xregs;
        let fregs = &mut self.fregs;

        log(&format!(
            "execute pc: {} ({:#x}), opcode: {} ({:#x}, {:#b}), binary: {:#x}",
            self.pc, self.pc, opcode, opcode, opcode, binary
        ));

        match opcode {
            0x03 => {
                // I-type
                let offset = (((binary & 0xFFF00000) as i32) as i64) >> 20;
                let addr = (xregs[rs1] + offset) as usize;
                match funct3 {
                    0x0 => xregs[rd] = (get_memory8(addr, mem) as i8) as i64, // lb
                    0x1 => xregs[rd] = (get_memory16(addr, mem) as i16) as i64, // lh
                    0x2 => xregs[rd] = (get_memory32(addr, mem) as i32) as i64, // lw
                    0x3 => xregs[rd] = get_memory64(addr, mem) as i64,        // ld
                    0x4 => xregs[rd] = (get_memory8(addr, mem) as i64) & 0xFF, // lbu
                    0x5 => xregs[rd] = (get_memory16(addr, mem) as i64) & 0xFFFF, // lhu
                    0x6 => xregs[rd] = (get_memory32(addr, mem) as i64) & 0xFFFFFFFF, // lwu
                    _ => {}
                }
            }
            0x07 => {
                // I-type (RV32F and RV64F)
                if funct3 != 0b010 {
                    let text = format!(
                        "the width of FLW should be 0b010 but got {:#b} ({})",
                        funct3, funct3
                    );
                    log(&text);
                    render(&text);
                    exit(1);
                }
                let offset = (((binary & 0xFFF00000) as i32) as i64) >> 20;
                let addr = (xregs[rs1] + offset) as usize;
                fregs[rd] = f32::from_bits(get_memory32(addr, mem)); // flw
            }
            0x0F => {
                // I-type
                // fence instructions are not supportted yet because this emulator executes a
                // binary sequentially on a single thread.
                // fence i is a part of the Zifencei extension.
                match funct3 {
                    0x0 => {} // fence
                    0x1 => {} // fence.i
                    _ => {}
                }
            }
            0x13 => {
                // I-type
                let imm = (((binary & 0xFFF00000) as i32) as i64) >> 20;
                // shamt size is 5 bits for RV32I and 6 bits for RV64I.
                // let shamt = (binary & 0x01F00000) >> 20; // This is for RV32I
                let shamt = (binary & 0x03F00000) >> 20;
                let funct6 = funct7 >> 1;
                match funct3 {
                    0x0 => xregs[rd] = xregs[rs1].wrapping_add(imm), // addi
                    0x1 => xregs[rd] = ((xregs[rs1] as u64) << shamt) as i64, // slli
                    0x2 => xregs[rd] = if xregs[rs1] < imm { 1 } else { 0 }, // slti
                    0x3 => {
                        xregs[rd] = if (xregs[rs1] as u64) < (imm as u64) {
                            1
                        } else {
                            0
                        }
                    } // sltiu
                    0x4 => xregs[rd] = xregs[rs1] ^ imm,             // xori
                    0x5 => {
                        match funct6 {
                            0x00 => xregs[rd] = ((xregs[rs1] as u64) >> shamt) as i64, // srli
                            0x10 => xregs[rd] = xregs[rs1] >> shamt,                   // srai
                            _ => {}
                        }
                    }
                    0x6 => xregs[rd] = xregs[rs1] | imm, // ori
                    0x7 => xregs[rd] = xregs[rs1] & imm, // andi
                    _ => {}
                }
            }
            0x17 => {
                // U-type
                // AUIPC forms a 32-bit offset from the 20-bit U-immediate, filling
                // in the lowest 12 bits with zeros.
                let imm = ((binary & 0xFFFFF000) as i32) as i64;
                xregs[rd] = (self.pc as i64) + imm; // auipc
            }
            0x1B => {
                // I-type (RV64I only)
                let imm = (((binary & 0xFFF00000) as i32) as i64) >> 20;
                let shamt = (binary & 0x01F00000) >> 20;
                match funct3 {
                    0x0 => {
                        xregs[rd] = (((xregs[rs1].wrapping_add(imm)) & 0xFFFFFFFF) as i32) as i64
                    } // addiw
                    0x1 => xregs[rd] = (((xregs[rs1] << shamt) & 0xFFFFFFFF) as i32) as i64, // slliw
                    0x5 => {
                        match funct7 {
                            0x00 => xregs[rd] = ((xregs[rs1] as u32) >> shamt) as i64, // srliw
                            0x20 => xregs[rd] = ((xregs[rs1] as i32) >> shamt) as i64, // sraiw
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            0x23 => {
                // S-type
                let imm11_5 = (((binary & 0xFE000000) as i32) as i64) >> 25;
                let imm4_0 = ((binary & 0x00000F80) >> 7) as u64;
                let offset = (((imm11_5 << 5) as u64) | imm4_0) as i64;
                let addr = (xregs[rs1] + offset) as usize;
                match funct3 {
                    0x0 => set_memory8(addr, mem, xregs[rs2] as u8), // sb
                    0x1 => set_memory16(addr, mem, xregs[rs2] as u16), // sh
                    0x2 => set_memory32(addr, mem, xregs[rs2] as u32), // sw
                    0x3 => set_memory64(addr, mem, xregs[rs2] as u64), // sd
                    _ => {}
                }
            }
            0x27 => {
                // S-type (RV32F and RV64F)
                if funct3 != 0b010 {
                    let text = format!(
                        "the width of FSW should be 0b010 but got {:#b} ({})",
                        funct3, funct3
                    );
                    log(&text);
                    render(&text);
                    exit(1);
                }
                let imm11_5 = (((binary & 0xFE000000) as i32) as i64) >> 25;
                let imm4_0 = ((binary & 0x00000F80) >> 7) as u64;
                let offset = (((imm11_5 << 5) as u64) | imm4_0) as i64;
                let addr = (xregs[rs1] + offset) as usize;
                set_memory32(addr, mem, fregs[rs2].to_bits()); // fsw
            }
            0x33 => {
                // R-type (RV32I and RV32M)
                let shamt = xregs[rs2] as u64;
                match (funct3, funct7) {
                    (0x0, 0x00) => xregs[rd] = xregs[rs1].wrapping_add(xregs[rs2]), // add
                    (0x0, 0x01) => xregs[rd] = xregs[rs1].wrapping_mul(xregs[rs2]), // mul
                    (0x0, 0x20) => xregs[rd] = xregs[rs1].wrapping_sub(xregs[rs2]), // sub
                    (0x1, 0x00) => xregs[rd] = ((xregs[rs1] as u64) << shamt) as i64, // sll
                    (0x1, 0x01) => {
                        // mulh
                        let n1 = BigInt::from(xregs[rs1]);
                        let n2 = BigInt::from(xregs[rs2]);
                        xregs[rd] = ((n1 * n2) >> 64).to_i64().unwrap();
                    }
                    (0x2, 0x00) => xregs[rd] = if xregs[rs1] < xregs[rs2] { 1 } else { 0 }, // slt
                    (0x2, 0x01) => {
                        // mulhsu
                        // get the most significant bit
                        let sign = ((xregs[rs1] as u64) & 0x80000000_00000000) as i64;
                        // xregs[rs1] is signed and xregs[rs2] is unsigned
                        let n1 = BigUint::from((xregs[rs1] as u64) & 0xefffffff_ffffffff);
                        let n2 = BigUint::from(xregs[rs2] as u64);
                        xregs[rd] = sign | ((n1 * n2) >> 64).to_i64().unwrap();
                    }
                    (0x3, 0x00) => {
                        xregs[rd] = if (xregs[rs1] as u64) < (xregs[rs2] as u64) {
                            1
                        } else {
                            0
                        }
                    } // sltu
                    (0x3, 0x01) => {
                        // mulhu
                        let n1 = BigUint::from(xregs[rs1] as u64);
                        let n2 = BigUint::from(xregs[rs2] as u64);
                        xregs[rd] = ((n1 * n2) >> 64).to_i64().unwrap();
                    }
                    (0x4, 0x00) => xregs[rd] = xregs[rs1] ^ xregs[rs2], // xor
                    (0x4, 0x01) => xregs[rd] = xregs[rs1].wrapping_div(xregs[rs2]), // div
                    (0x5, 0x00) => xregs[rd] = ((xregs[rs1] as u64) >> shamt) as i64, // srl
                    (0x5, 0x01) => {
                        // divu
                        let dividend = xregs[rs1] as u64;
                        let divisor = xregs[rs2] as u64;
                        xregs[rd] = dividend.wrapping_div(divisor) as i64;
                    }
                    (0x5, 0x20) => xregs[rd] = xregs[rs1] >> shamt, // sra
                    (0x6, 0x00) => xregs[rd] = xregs[rs1] | xregs[rs2], // or
                    (0x6, 0x01) => xregs[rd] = xregs[rs1] % xregs[rs2], // rem
                    (0x7, 0x00) => xregs[rd] = xregs[rs1] & xregs[rs2], // and
                    (0x7, 0x01) => {
                        // remu
                        let dividend = xregs[rs1] as u64;
                        let divisor = xregs[rs2] as u64;
                        xregs[rd] = (dividend % divisor) as i64;
                    }
                    _ => {}
                };
            }
            0x37 => {
                // U-type
                // LUI places the U-immediate value in the top 20 bits of the destination
                // register rd, filling in the lowest 12 bits with zeros.
                xregs[rd] = ((binary & 0xFFFFF000) as i32) as i64; // lui
            }
            0x3B => {
                // R-type (RV64I and RV64M)
                // The shift amount is given by rs2[4:0].
                let shamt = (xregs[rs2] & 0x1F) as u32;
                match (funct3, funct7) {
                    (0x0, 0x00) => {
                        xregs[rd] = ((xregs[rs1].wrapping_add(xregs[rs2])) as i32) as i64
                    } // addw
                    (0x0, 0x01) => {
                        // mulw
                        let n1 = xregs[rs1] as i32;
                        let n2 = xregs[rs2] as i32;
                        let result = n1.wrapping_mul(n2);
                        xregs[rd] = result as i64;
                    }
                    (0x0, 0x20) => {
                        xregs[rd] = ((xregs[rs1].wrapping_sub(xregs[rs2])) as i32) as i64
                    } // subw
                    (0x1, 0x00) => xregs[rd] = (((xregs[rs1] as u32) << shamt) as i32) as i64, // sllw
                    (0x4, 0x01) => {
                        // divw
                        let dividend = xregs[rs1] as i32;
                        let divisor = xregs[rs2] as i32;
                        xregs[rd] = dividend.wrapping_div(divisor) as i64;
                    }
                    (0x5, 0x00) => xregs[rd] = (((xregs[rs1] as u32) >> shamt) as i32) as i64, // srlw
                    (0x5, 0x01) => {
                        // divuw
                        let dividend = xregs[rs1] as u32;
                        let divisor = xregs[rs2] as u32;
                        xregs[rd] = (dividend.wrapping_div(divisor) as i32) as i64;
                    }
                    (0x5, 0x20) => xregs[rd] = ((xregs[rs1] as i32) >> (shamt as i32)) as i64, // sraw
                    (0x6, 0x01) => {
                        // remw
                        let dividend = xregs[rs1] as i32;
                        let divisor = xregs[rs2] as i32;
                        xregs[rd] = dividend.wrapping_rem(divisor) as i64;
                    }
                    (0x7, 0x01) => {
                        // remuw
                        let dividend = xregs[rs1] as u32;
                        let divisor = xregs[rs2] as u32;
                        xregs[rd] = (dividend.wrapping_rem(divisor) as i32) as i64;
                    }
                    _ => {}
                }
            }
            0x43 => {
                // R4-type (RV32F and RV64F)
                // TODO: support the rounding mode encoding (rm). Currently only "000 RNE Round to Nearest, ties to
                // Even" is supported.
                let rs3 = ((binary & 0xF8000000) >> 27) as usize;
                fregs[rd] = fregs[rs1] * fregs[rs2] + fregs[rs3]; // fmadd.s
            }
            0x47 => {
                // R4-type (RV32F and RV64F)
                // TODO: support the rounding mode encoding (rm). Currently only "000 RNE Round to Nearest, ties to
                // Even" is supported.
                let rs3 = ((binary & 0xF8000000) >> 27) as usize;
                fregs[rd] = fregs[rs1] * fregs[rs2] - fregs[rs3]; // fmsub.s
            }
            0x4B => {
                // R4-type (RV32F and RV64F)
                // TODO: support the rounding mode encoding (rm). Currently only "000 RNE Round to Nearest, ties to
                // Even" is supported.
                let rs3 = ((binary & 0xF8000000) >> 27) as usize;
                fregs[rd] = -(fregs[rs1] * fregs[rs2]) + fregs[rs3]; // fnmadd.s
            }
            0x4F => {
                // R4-type (RV32F and RV64F)
                // TODO: support the rounding mode encoding (rm). Currently only "000 RNE Round to Nearest, ties to
                // Even" is supported.
                let rs3 = ((binary & 0xF8000000) >> 27) as usize;
                fregs[rd] = -(fregs[rs1] * fregs[rs2]) - fregs[rs3]; // fnmsub.s
            }
            0x53 => {
                // R-type (RV32F and RV64F)
                match funct7 {
                    // TODO: support the rounding mode encoding (rm). Currently only "000 RNE Round to Nearest, ties to
                    // Even" is supported.
                    0x00 => fregs[rd] = fregs[rs1] + fregs[rs2], // fadd.s
                    0x04 => fregs[rd] = fregs[rs1] - fregs[rs2], // fsub.s
                    0x08 => fregs[rd] = fregs[rs1] * fregs[rs2], // fmul.s
                    0x0c => fregs[rd] = fregs[rs1] / fregs[rs2], // fdiv.s
                    0x14 => {
                        match funct3 {
                            0x0 => fregs[rd] = fregs[rs1].min(fregs[rs2]), // fmin.s
                            0x1 => fregs[rd] = fregs[rs1].max(fregs[rs2]), // fmax.s
                            _ => {}
                        }
                    }
                    0x2c => fregs[rd] = fregs[rs1].sqrt(), // fsqrt.s
                    _ => {}
                }
            }
            0x63 => {
                // B-type
                let imm12 = (((binary & 0x80000000) as i32) as i64) >> 31;
                let imm10_5 = ((binary & 0x7E000000) >> 25) as u64;
                let imm4_1 = ((binary & 0x00000F00) >> 8) as u64;
                let imm11 = ((binary & 0x00000080) >> 7) as u64;
                let offset =
                    ((imm12 << 12) as u64 | (imm11 << 11) | (imm10_5 << 5) | (imm4_1 << 1)) as i64;
                match funct3 {
                    0x0 => {
                        // beq
                        if xregs[rs1] == xregs[rs2] {
                            self.pc = ((self.pc as i64) + offset - 4) as usize;
                        }
                    }
                    0x1 => {
                        // bne
                        if xregs[rs1] != xregs[rs2] {
                            self.pc = ((self.pc as i64) + offset - 4) as usize;
                        }
                    }
                    0x4 => {
                        // blt
                        if xregs[rs1] < xregs[rs2] {
                            self.pc = ((self.pc as i64) + offset - 4) as usize;
                        }
                    }
                    0x5 => {
                        // bge
                        if xregs[rs1] >= xregs[rs2] {
                            self.pc = ((self.pc as i64) + offset - 4) as usize;
                        }
                    }
                    0x6 => {
                        // bltu
                        if (xregs[rs1] as u64) < (xregs[rs2] as u64) {
                            self.pc = ((self.pc as i64) + offset - 4) as usize;
                        }
                    }
                    0x7 => {
                        // bgeu
                        if (xregs[rs1] as u64) >= (xregs[rs2] as u64) {
                            self.pc = ((self.pc as i64) + offset - 4) as usize;
                        }
                    }
                    _ => {}
                }
            }
            0x67 => {
                // I-type
                // jalr
                xregs[rd] = (self.pc as i64) + 4;

                let imm = (((binary & 0xFFF00000) as i32) as i64) >> 20;
                self.pc = ((xregs[rs1] + imm - 4) & !1) as usize;
            }
            0x6F => {
                // J-type
                // jal
                xregs[rd] = (self.pc as i64) + 4;

                let imm20 = (((binary & 0x80000000) as i32) as i64) >> 31;
                let imm10_1 = ((binary & 0x7FE00000) >> 21) as u64;
                let imm11 = ((binary & 0x100000) >> 20) as u64;
                let imm19_12 = ((binary & 0xFF000) >> 12) as u64;
                let offset =
                    ((imm20 << 20) as u64 | (imm19_12 << 12) | (imm11 << 11) | (imm10_1 << 1))
                        as i64;
                let tmp = (self.pc as i64) + offset - 4;
                self.pc = tmp as usize;
            }
            0x73 => {
                // I-type
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
                            0x0 => {} // ecall
                            0x1 => {} // ebreak
                            _ => {}
                        }
                    }
                    // TODO: implement RV32/RV64 Zicsr Standard Extension
                    0x1 => {} // csrrw
                    0x2 => {} // csrrs
                    0x3 => {} // csrrc
                    0x5 => {} // csrrwi
                    0x6 => {} // csrrsi
                    0x7 => {} // csrrci
                    _ => {}
                }
            }
            _ => {
                let text = format!(
                    "not implemented opcode {} ({:#x}, {:#b})",
                    opcode, opcode, opcode
                );
                log(&text);
                render(&text);
                exit(1);
            }
        }
    }
}
