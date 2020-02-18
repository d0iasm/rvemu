pub const REGISTERS_COUNT: usize = 32;

use std::cmp;
use std::num::FpCategory;
use std::process::exit;

use num_bigint::{BigInt, BigUint};
use num_traits::cast::ToPrimitive;

use crate::*;
use fcsr::*;

pub enum Mode {
    User,
    Supervisor,
    Machine,
    Debug,
}

pub struct Cpu {
    pub xregs: [i64; REGISTERS_COUNT],
    pub fregs: [f64; REGISTERS_COUNT],
    pub pc: usize,
    pub csr: Csr,
    pub mode: Mode,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            xregs: [0; REGISTERS_COUNT],
            fregs: [0.0; REGISTERS_COUNT],
            pc: 0,
            csr: Csr::new(),
            mode: Mode::Machine,
        }
    }

    /// Reset CPU states.
    pub fn reset(&mut self) {
        self.pc = 0;
        self.csr.clear();
        // TODO: reset CPU mode to machine or not?
        for i in 0..REGISTERS_COUNT {
            self.xregs[i] = 0;
            self.fregs[i] = 0.0;
        }
    }

    /// Start executing the CPU.
    pub fn start(&mut self, mem: &mut Memory) {
        let size = mem.len();

        let mut i = 0;
        while self.pc < size {
            let binary = self.fetch(mem);
            let _ = self.execute(binary, mem).map_err(|e| e.take_trap(self));
            self.pc += 4;

            // TODO: Remove the following check.
            // This is for avoiding an infinite execution.
            i += 1;
            if i > 1000 {
                exit(1);
            }
        }
    }

    /// Fetch the next instruction from a memory at the current program counter.
    fn fetch(&mut self, mem: &Memory) -> u32 {
        mem.read32(self.pc)
    }

    /// Execute an instruction.
    // This function is public because it's called from a unit test.
    pub fn execute(&mut self, binary: u32, mem: &mut Memory) -> Result<(), Exception> {
        let opcode = binary & 0x0000007f;
        let rd = ((binary & 0x00000f80) >> 7) as usize;
        let rs1 = ((binary & 0x000f8000) >> 15) as usize;
        let rs2 = ((binary & 0x01f00000) >> 20) as usize;
        let funct3 = (binary & 0x00007000) >> 12;
        let funct7 = (binary & 0xfe000000) >> 25;

        let xregs = &mut self.xregs;
        let fregs = &mut self.fregs;

        log(&format!(
            "execute pc: {} ({:#x}), opcode: {} ({:#x}, {:#b}), binary: {:#x}",
            self.pc, self.pc, opcode, opcode, opcode, binary
        ));

        match opcode {
            0x03 => {
                // I-type
                let offset = (((binary & 0xfff00000) as i32) as i64) >> 20;
                let addr = (xregs[rs1] + offset) as usize;
                match funct3 {
                    0x0 => xregs[rd] = (mem.read8(addr) as i8) as i64, // lb
                    0x1 => xregs[rd] = (mem.read16(addr) as i16) as i64, // lh
                    0x2 => xregs[rd] = (mem.read32(addr) as i32) as i64, // lw
                    0x3 => xregs[rd] = mem.read64(addr) as i64,        // ld
                    0x4 => xregs[rd] = (mem.read8(addr) as i64) & 0xff, // lbu
                    0x5 => xregs[rd] = (mem.read16(addr) as i64) & 0xffff, // lhu
                    0x6 => xregs[rd] = (mem.read32(addr) as i64) & 0xffffffff, // lwu
                    _ => {}
                }
            }
            0x07 => {
                // I-type (RV32F and RV64F)
                let offset = (((binary & 0xfff00000) as i32) as i64) >> 20;
                let addr = (xregs[rs1] + offset) as usize;
                match funct3 {
                    0x2 => fregs[rd] = f64::from_bits(mem.read32(addr) as u64), // flw
                    0x3 => fregs[rd] = f64::from_bits(mem.read64(addr)),        // fld
                    _ => {}
                }
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
                let imm = (((binary & 0xfff00000) as i32) as i64) >> 20;
                // shamt size is 5 bits for RV32I and 6 bits for RV64I.
                // let shamt = (binary & 0x01F00000) >> 20; // This is for RV32I
                let shamt = (binary & 0x03f00000) >> 20;
                let funct6 = funct7 >> 1;
                match funct3 {
                    0x0 => xregs[rd] = xregs[rs1].wrapping_add(imm), // addi
                    0x1 => xregs[rd] = ((xregs[rs1] as u64) << shamt) as i64, // slli
                    0x2 => xregs[rd] = if xregs[rs1] < imm { 1 } else { 0 }, // slti
                    0x3 => {
                        // sltiu
                        xregs[rd] = if (xregs[rs1] as u64) < (imm as u64) {
                            1
                        } else {
                            0
                        }
                    }
                    0x4 => xregs[rd] = xregs[rs1] ^ imm, // xori
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
                let imm = ((binary & 0xfffff000) as i32) as i64;
                xregs[rd] = (self.pc as i64) + imm; // auipc
            }
            0x1B => {
                // I-type (RV64I only)
                let imm = (((binary & 0xfff00000) as i32) as i64) >> 20;
                let shamt = (binary & 0x01f00000) >> 20;
                match funct3 {
                    0x0 => {
                        // addiw
                        xregs[rd] = (((xregs[rs1].wrapping_add(imm)) & 0xffffffff) as i32) as i64
                    }
                    0x1 => xregs[rd] = (((xregs[rs1] << shamt) & 0xffffffff) as i32) as i64, // slliw
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
                let imm11_5 = (((binary & 0xfe000000) as i32) as i64) >> 25;
                let imm4_0 = ((binary & 0x00000f80) >> 7) as u64;
                let offset = (((imm11_5 << 5) as u64) | imm4_0) as i64;
                let addr = (xregs[rs1] + offset) as usize;
                match funct3 {
                    0x0 => mem.write8(addr, xregs[rs2] as u8),   // sb
                    0x1 => mem.write16(addr, xregs[rs2] as u16), // sh
                    0x2 => mem.write32(addr, xregs[rs2] as u32), // sw
                    0x3 => mem.write64(addr, xregs[rs2] as u64), // sd
                    _ => {}
                }
            }
            0x27 => {
                // S-type (RV32F and RV64F)
                let imm11_5 = (((binary & 0xfe000000) as i32) as i64) >> 25;
                let imm4_0 = ((binary & 0x00000f80) >> 7) as u64;
                let offset = (((imm11_5 << 5) as u64) | imm4_0) as i64;
                let addr = (xregs[rs1] + offset) as usize;
                match funct3 {
                    0x2 => mem.write32(addr, (fregs[rs2] as f32).to_bits()), // fsw
                    0x3 => mem.write64(addr, fregs[rs2].to_bits()),          // fsd
                    _ => {}
                }
            }
            0x2F => {
                // R-type (RV32A and RV64A)
                let funct5 = (funct7 & 0b1111100) >> 2;
                let _aq = (funct7 & 0b0000010) >> 1; // acquire access
                let _rl = funct7 & 0b0000001; // release access
                match (funct3, funct5) {
                    // TODO: if the address is not naturally aligned, a misaligned address
                    // exception or an access exception will be generated.
                    (0x2, 0x00) => {
                        // amoadd.w
                        let t = mem.read32(xregs[rs1] as usize) as i32;
                        mem.write32(
                            xregs[rs1] as usize,
                            (t.wrapping_add(xregs[rs2] as i32)) as u32,
                        );
                        xregs[rd] = t as i64;
                    }
                    (0x3, 0x00) => {
                        // amoadd.d
                        let t = mem.read64(xregs[rs1] as usize) as i64;
                        mem.write64(xregs[rs1] as usize, t.wrapping_add(xregs[rs2]) as u64);
                        xregs[rd] = t;
                    }
                    (0x2, 0x01) => {
                        // amoswap.w
                        let t = mem.read32(xregs[rs1] as usize) as i32;
                        mem.write32(xregs[rs1] as usize, xregs[rs2] as u32);
                        xregs[rd] = t as i64;
                    }
                    (0x3, 0x01) => {
                        // amoswap.d
                        let t = mem.read64(xregs[rs1] as usize) as i64;
                        mem.write64(xregs[rs1] as usize, xregs[rs2] as u64);
                        xregs[rd] = t;
                    }
                    (0x2, 0x02) => xregs[rd] = (mem.read32(xregs[rs1] as usize) as i32) as i64, // lr.w
                    (0x3, 0x02) => xregs[rd] = mem.read64(xregs[rs1] as usize) as i64, // lr.d
                    (0x2, 0x03) => {
                        // TODO: write a nonzero error code if the store fails.
                        // sc.w
                        xregs[rd] = 0;
                        mem.write32(xregs[rs1] as usize, xregs[rs2] as u32);
                    }
                    (0x3, 0x03) => {
                        // TODO: write a nonzero error code if the store fails.
                        // sc.d
                        xregs[rd] = 0;
                        mem.write64(xregs[rs1] as usize, xregs[rs2] as u64);
                    }
                    (0x2, 0x04) => {
                        // amoxor.w
                        let t = mem.read32(xregs[rs1] as usize) as i32;
                        mem.write32(xregs[rs1] as usize, (t ^ (xregs[rs2] as i32)) as u32);
                        xregs[rd] = t as i64;
                    }
                    (0x3, 0x04) => {
                        // amoxor.d
                        let t = mem.read64(xregs[rs1] as usize) as i64;
                        mem.write64(xregs[rs1] as usize, (t ^ xregs[rs2]) as u64);
                        xregs[rd] = t;
                    }
                    (0x2, 0x08) => {
                        // amoor.w
                        let t = mem.read32(xregs[rs1] as usize) as i32;
                        mem.write32(xregs[rs1] as usize, (t | (xregs[rs2] as i32)) as u32);
                        xregs[rd] = t as i64;
                    }
                    (0x3, 0x08) => {
                        // amoor.d
                        let t = mem.read64(xregs[rs1] as usize) as i64;
                        mem.write64(xregs[rs1] as usize, (t | xregs[rs2]) as u64);
                        xregs[rd] = t;
                    }
                    (0x2, 0x0c) => {
                        // amoand.w
                        let t = mem.read32(xregs[rs1] as usize) as i32;
                        mem.write32(xregs[rs1] as usize, (t & (xregs[rs2] as i32)) as u32);
                        xregs[rd] = t as i64;
                    }
                    (0x3, 0x0c) => {
                        // amoand.d
                        let t = mem.read64(xregs[rs1] as usize) as i64;
                        mem.write64(xregs[rs1] as usize, (t & xregs[rs1]) as u64);
                        xregs[rd] = t;
                    }
                    (0x2, 0x10) => {
                        // amomin.w
                        let t = mem.read32(xregs[rs1] as usize) as i32;
                        mem.write32(xregs[rs1] as usize, cmp::min(t, xregs[rs2] as i32) as u32);
                        xregs[rd] = t as i64;
                    }
                    (0x3, 0x10) => {
                        // amomin.d
                        let t = mem.read64(xregs[rs1] as usize) as i64;
                        mem.write64(xregs[rs1] as usize, cmp::min(t, xregs[rs2]) as u64);
                        xregs[rd] = t;
                    }
                    (0x2, 0x14) => {
                        // amomax.w
                        let t = mem.read32(xregs[rs1] as usize) as i32;
                        mem.write32(xregs[rs1] as usize, cmp::max(t, xregs[rs2] as i32) as u32);
                        xregs[rd] = t as i64;
                    }
                    (0x3, 0x14) => {
                        // amomax.d
                        let t = mem.read64(xregs[rs1] as usize) as i64;
                        mem.write64(xregs[rs1] as usize, cmp::max(t, xregs[rs2]) as u64);
                        xregs[rd] = t;
                    }
                    (0x2, 0x18) => {
                        // amominu.w
                        let t = mem.read32(xregs[rs1] as usize);
                        mem.write32(xregs[rs1] as usize, cmp::min(t, xregs[rs2] as u32));
                        xregs[rd] = (t as i32) as i64;
                    }
                    (0x3, 0x18) => {
                        // amominu.d
                        let t = mem.read64(xregs[rs1] as usize);
                        mem.write64(xregs[rs1] as usize, cmp::min(t, xregs[rs2] as u64));
                        xregs[rd] = t as i64;
                    }
                    (0x2, 0x1c) => {
                        // amomaxu.w
                        let t = mem.read32(xregs[rs1] as usize);
                        mem.write32(xregs[rs1] as usize, cmp::max(t, xregs[rs2] as u32));
                        xregs[rd] = (t as i32) as i64;
                    }
                    (0x3, 0x1c) => {
                        // amomaxu.d
                        let t = mem.read64(xregs[rs1] as usize);
                        mem.write64(xregs[rs1] as usize, cmp::max(t, xregs[rs2] as u64));
                        xregs[rd] = t as i64;
                    }
                    _ => {}
                }
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
                        // sltu
                        xregs[rd] = if (xregs[rs1] as u64) < (xregs[rs2] as u64) {
                            1
                        } else {
                            0
                        }
                    }
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
                xregs[rd] = ((binary & 0xfffff000) as i32) as i64; // lui
            }
            0x3B => {
                // R-type (RV64I and RV64M)
                // The shift amount is given by rs2[4:0].
                let shamt = (xregs[rs2] & 0x1f) as u32;
                match (funct3, funct7) {
                    (0x0, 0x00) => {
                        // addw
                        xregs[rd] = ((xregs[rs1].wrapping_add(xregs[rs2])) as i32) as i64
                    }
                    (0x0, 0x01) => {
                        // mulw
                        let n1 = xregs[rs1] as i32;
                        let n2 = xregs[rs2] as i32;
                        let result = n1.wrapping_mul(n2);
                        xregs[rd] = result as i64;
                    }
                    (0x0, 0x20) => {
                        // subw
                        xregs[rd] = ((xregs[rs1].wrapping_sub(xregs[rs2])) as i32) as i64
                    }
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
                // TODO: support the rounding mode encoding (rm).
                let rs3 = ((binary & 0xf8000000) >> 27) as usize;
                let funct2 = (binary & 0x03000000) >> 25;
                match funct2 {
                    0x0 => {
                        // fmadd.s
                        fregs[rd] =
                            (fregs[rs1] as f32).mul_add(fregs[rs2] as f32, fregs[rs3] as f32) as f64
                    }
                    0x1 => fregs[rd] = fregs[rs1].mul_add(fregs[rs2], fregs[rs3]), // fmadd.d
                    _ => {}
                }
            }
            0x47 => {
                // R4-type (RV32F and RV64F)
                // TODO: support the rounding mode encoding (rm).
                let rs3 = ((binary & 0xf8000000) >> 27) as usize;
                let funct2 = (binary & 0x03000000) >> 25;
                match funct2 {
                    0x0 => {
                        // fmsub.s
                        fregs[rd] = (fregs[rs1] as f32)
                            .mul_add(fregs[rs2] as f32, -fregs[rs3] as f32)
                            as f64
                    }
                    0x1 => fregs[rd] = fregs[rs1].mul_add(fregs[rs2], -fregs[rs3]), // fmsub.d
                    _ => {}
                }
            }
            0x4B => {
                // R4-type (RV32F and RV64F)
                // TODO: support the rounding mode encoding (rm).
                let rs3 = ((binary & 0xf8000000) >> 27) as usize;
                let funct2 = (binary & 0x03000000) >> 25;
                match funct2 {
                    0x0 => {
                        // fnmadd.s
                        fregs[rd] = (-fregs[rs1] as f32)
                            .mul_add(fregs[rs2] as f32, fregs[rs3] as f32)
                            as f64
                    }
                    0x1 => fregs[rd] = (-fregs[rs1]).mul_add(fregs[rs2], fregs[rs3]), // fnmadd.d
                    _ => {}
                }
            }
            0x4F => {
                // R4-type (RV32F and RV64F)
                // TODO: support the rounding mode encoding (rm).
                let rs3 = ((binary & 0xf8000000) >> 27) as usize;
                let funct2 = (binary & 0x03000000) >> 25;
                match funct2 {
                    0x0 => {
                        // fnmsub.s
                        fregs[rd] = (-fregs[rs1] as f32)
                            .mul_add(fregs[rs2] as f32, -fregs[rs3] as f32)
                            as f64
                    }
                    0x1 => fregs[rd] = (-fregs[rs1]).mul_add(fregs[rs2], -fregs[rs3]), // fnmsub.d
                    _ => {}
                }
            }
            0x53 => {
                // R-type (RV32F and RV64F)
                // TODO: support the rounding mode encoding (rm).
                // TODO: NaN Boxing of Narrower Values (Spec 12.2).
                // TODO: set exception flags.

                /*
                 * Floating-point instructions align with the IEEE 754 (1985).
                 * The format consist of three fields: a sign bit, a biased exponent, and a fraction.
                 *
                 * | sign(1) | exponent(8) | fraction(23) |
                 * 31                                     0
                 *
                 */

                let fcsr = self.csr.read(FCSR)? as u64 as u32;
                let frm = Fcsr::from_bits(fcsr)
                    .expect("failed to convert fcsr")
                    .get_rounding_mode();
                if frm == RoundingMode::Invalid {
                    return Err(Exception::IllegalInstruction(String::from(
                        "frm is set to an invalid value (101–110)",
                    )));
                }

                match funct7 {
                    0x00 => fregs[rd] = (fregs[rs1] as f32 + fregs[rs2] as f32) as f64, // fadd.s
                    0x01 => fregs[rd] = fregs[rs1] + fregs[rs2],                        // fadd.d
                    0x04 => fregs[rd] = (fregs[rs1] as f32 - fregs[rs2] as f32) as f64, // fsub.s
                    0x05 => fregs[rd] = fregs[rs1] - fregs[rs2],                        // fsub.d
                    0x08 => fregs[rd] = (fregs[rs1] as f32 * fregs[rs2] as f32) as f64, // fmul.s
                    0x09 => fregs[rd] = fregs[rs1] * fregs[rs2],                        // fmul.d
                    0x0c => fregs[rd] = (fregs[rs1] as f32 / fregs[rs2] as f32) as f64, // fdiv.s
                    0x0d => fregs[rd] = fregs[rs1] / fregs[rs2],                        // fdiv.d
                    0x10 => {
                        match funct3 {
                            0x0 => fregs[rd] = fregs[rs1].copysign(fregs[rs2]), // fsgnj.s
                            0x1 => fregs[rd] = fregs[rs1].copysign(-fregs[rs2]), // fsgnjn.s
                            0x2 => {
                                let sign1 = (fregs[rs1] as f32).to_bits() & 0x80000000;
                                let sign2 = (fregs[rs2] as f32).to_bits() & 0x80000000;
                                let other = (fregs[rs1] as f32).to_bits() & 0x7fffffff;
                                // fsgnjx.s
                                fregs[rd] = f32::from_bits((sign1 ^ sign2) | other) as f64;
                            }
                            _ => {}
                        }
                    }
                    0x11 => {
                        match funct3 {
                            0x0 => fregs[rd] = fregs[rs1].copysign(fregs[rs2]), // fsgnj.d
                            0x1 => fregs[rd] = fregs[rs1].copysign(-fregs[rs2]), // fsgnjn.d
                            0x2 => {
                                let sign1 = fregs[rs1].to_bits() & 0x80000000_00000000;
                                let sign2 = fregs[rs2].to_bits() & 0x80000000_00000000;
                                let other = fregs[rs1].to_bits() & 0x7fffffff_ffffffff;
                                // fsgnjx.d
                                fregs[rd] = f64::from_bits((sign1 ^ sign2) | other);
                            }
                            _ => {}
                        }
                    }
                    0x14 => {
                        match funct3 {
                            0x0 => fregs[rd] = fregs[rs1].min(fregs[rs2]), // fmin.s
                            0x1 => fregs[rd] = fregs[rs1].max(fregs[rs2]), // fmax.s
                            _ => {}
                        }
                    }
                    0x15 => {
                        match funct3 {
                            0x0 => fregs[rd] = fregs[rs1].min(fregs[rs2]), // fmin.d
                            0x1 => fregs[rd] = fregs[rs1].max(fregs[rs2]), // fmax.d
                            _ => {}
                        }
                    }
                    0x20 => fregs[rd] = fregs[rs1], // fcvt.s.d
                    0x21 => fregs[rd] = (fregs[rs1] as f32) as f64, // fcvt.d.s
                    0x2c => fregs[rd] = (fregs[rs1] as f32).sqrt() as f64, // fsqrt.s
                    0x2d => fregs[rd] = fregs[rs1].sqrt(), // fsqrt.d
                    0x50 => {
                        match funct3 {
                            0x0 => xregs[rd] = if fregs[rs1] <= fregs[rs2] { 1 } else { 0 }, // fle.s
                            0x1 => xregs[rd] = if fregs[rs1] < fregs[rs2] { 1 } else { 0 }, // flt.s
                            0x2 => xregs[rd] = if fregs[rs1] == fregs[rs2] { 1 } else { 0 }, // feq.s
                            _ => {}
                        }
                    }
                    0x51 => {
                        match funct3 {
                            0x0 => xregs[rd] = if fregs[rs1] <= fregs[rs2] { 1 } else { 0 }, // fle.d
                            0x1 => xregs[rd] = if fregs[rs1] < fregs[rs2] { 1 } else { 0 }, // flt.d
                            0x2 => xregs[rd] = if fregs[rs1] == fregs[rs2] { 1 } else { 0 }, // feq.d
                            _ => {}
                        }
                    }
                    0x60 => {
                        match rs2 {
                            0x0 => xregs[rd] = ((fregs[rs1] as f32).round() as i32) as i64, // fcvt.w.s
                            0x1 => xregs[rd] = (((fregs[rs1] as f32).round() as u32) as i32) as i64, // fcvt.wu.s
                            0x2 => xregs[rd] = (fregs[rs1] as f32).round() as i64, // fcvt.l.s
                            0x3 => xregs[rd] = ((fregs[rs1] as f32).round() as u64) as i64, // fcvt.lu.s
                            _ => {}
                        }
                    }
                    0x61 => {
                        match rs2 {
                            0x0 => xregs[rd] = (fregs[rs1].round() as i32) as i64, // fcvt.w.d
                            0x1 => xregs[rd] = ((fregs[rs1].round() as u32) as i32) as i64, // fcvt.wu.d
                            0x2 => xregs[rd] = fregs[rs1].round() as i64, // fcvt.l.d
                            0x3 => xregs[rd] = (fregs[rs1].round() as u64) as i64, // fcvt.lu.d
                            _ => {}
                        }
                    }
                    0x68 => {
                        match rs2 {
                            0x0 => fregs[rd] = ((xregs[rs1] as i32) as f32) as f64, // fcvt.s.w
                            0x1 => fregs[rd] = ((xregs[rs1] as u32) as f32) as f64, // fcvt.s.wu
                            0x2 => fregs[rd] = (xregs[rs1] as f32) as f64,          // fcvt.s.l
                            0x3 => fregs[rd] = ((xregs[rs1] as u64) as f32) as f64, // fcvt.s.lu
                            _ => {}
                        }
                    }
                    0x69 => {
                        match rs2 {
                            0x0 => fregs[rd] = (xregs[rs1] as i32) as f64, // fcvt.d.w
                            0x1 => fregs[rd] = (xregs[rs1] as u32) as f64, // fcvt.d.wu
                            0x2 => fregs[rd] = xregs[rs1] as f64,          // fcvt.d.l
                            0x3 => fregs[rd] = (xregs[rs1] as u64) as f64, // fcvt.d.lu
                            _ => {}
                        }
                    }
                    0x70 => {
                        match funct3 {
                            0x0 => xregs[rd] = (fregs[rs1] as i32) as i64, // fmv.x.w
                            0x1 => {
                                // fclass.s
                                let f = fregs[rs1];
                                match f.classify() {
                                    FpCategory::Infinite => {
                                        xregs[rd] = if f.is_sign_negative() { 0 } else { 7 }
                                    }
                                    FpCategory::Normal => {
                                        xregs[rd] = if f.is_sign_negative() { 1 } else { 6 }
                                    }
                                    FpCategory::Subnormal => {
                                        xregs[rd] = if f.is_sign_negative() { 2 } else { 5 }
                                    }
                                    FpCategory::Zero => {
                                        xregs[rd] = if f.is_sign_negative() { 3 } else { 4 }
                                    }
                                    // don't support a signaling nan, only support a quiet nan.
                                    FpCategory::Nan => xregs[rd] = 9,
                                }
                            }
                            _ => {}
                        }
                    }
                    0x71 => {
                        match funct3 {
                            0x0 => xregs[rd] = fregs[rs1] as i64, // fmv.x.d
                            0x1 => {
                                // fclass.d
                                let f = fregs[rs1];
                                match f.classify() {
                                    FpCategory::Infinite => {
                                        xregs[rd] = if f.is_sign_negative() { 0 } else { 7 }
                                    }
                                    FpCategory::Normal => {
                                        xregs[rd] = if f.is_sign_negative() { 1 } else { 6 }
                                    }
                                    FpCategory::Subnormal => {
                                        xregs[rd] = if f.is_sign_negative() { 2 } else { 5 }
                                    }
                                    FpCategory::Zero => {
                                        xregs[rd] = if f.is_sign_negative() { 3 } else { 4 }
                                    }
                                    // don't support a signaling nan, only support a quiet nan.
                                    FpCategory::Nan => xregs[rd] = 9,
                                }
                            }
                            _ => {}
                        }
                    }
                    0x78 => fregs[rd] = ((xregs[rs1] as i32) as f32) as f64, // fmv.w.x
                    0x79 => fregs[rd] = xregs[rs1] as f64,                   // fmv.d.x
                    _ => {}
                }
            }
            0x63 => {
                // B-type
                let imm12 = (((binary & 0x80000000) as i32) as i64) >> 31;
                let imm10_5 = ((binary & 0x7e000000) >> 25) as u64;
                let imm4_1 = ((binary & 0x00000f00) >> 8) as u64;
                let imm11 = ((binary & 0x00000080) >> 7) as u64;
                let offset =
                    ((imm12 << 12) as u64 | (imm11 << 11) | (imm10_5 << 5) | (imm4_1 << 1)) as i64;
                match funct3 {
                    0x0 => {
                        // beq
                        if xregs[rs1] == xregs[rs2] {
                            let target = (self.pc as i64) + offset - 4;
                            if target % 4 != 0 {
                                return Err(Exception::InstructionAddressMisaligned(String::from(
                                    "must be aligned on a four-byte boundary",
                                )));
                            }
                            self.pc = target as usize;
                        }
                    }
                    0x1 => {
                        // bne
                        if xregs[rs1] != xregs[rs2] {
                            let target = (self.pc as i64) + offset - 4;
                            if target % 4 != 0 {
                                return Err(Exception::InstructionAddressMisaligned(String::from(
                                    "must be aligned on a four-byte boundary",
                                )));
                            }
                            self.pc = target as usize;
                        }
                    }
                    0x4 => {
                        // blt
                        if xregs[rs1] < xregs[rs2] {
                            let target = (self.pc as i64) + offset - 4;
                            if target % 4 != 0 {
                                return Err(Exception::InstructionAddressMisaligned(String::from(
                                    "must be aligned on a four-byte boundary",
                                )));
                            }
                            self.pc = target as usize;
                        }
                    }
                    0x5 => {
                        // bge
                        if xregs[rs1] >= xregs[rs2] {
                            let target = (self.pc as i64) + offset - 4;
                            if target % 4 != 0 {
                                return Err(Exception::InstructionAddressMisaligned(String::from(
                                    "must be aligned on a four-byte boundary",
                                )));
                            }
                            self.pc = target as usize;
                        }
                    }
                    0x6 => {
                        // bltu
                        if (xregs[rs1] as u64) < (xregs[rs2] as u64) {
                            let target = (self.pc as i64) + offset - 4;
                            if target % 4 != 0 {
                                return Err(Exception::InstructionAddressMisaligned(String::from(
                                    "must be aligned on a four-byte boundary",
                                )));
                            }
                            self.pc = target as usize;
                        }
                    }
                    0x7 => {
                        // bgeu
                        if (xregs[rs1] as u64) >= (xregs[rs2] as u64) {
                            let target = (self.pc as i64) + offset - 4;
                            if target % 4 != 0 {
                                return Err(Exception::InstructionAddressMisaligned(String::from(
                                    "must be aligned on a four-byte boundary",
                                )));
                            }
                            self.pc = target as usize;
                        }
                    }
                    _ => {}
                }
            }
            0x67 => {
                // I-type
                // jalr
                xregs[rd] = (self.pc as i64) + 4;

                let imm = (((binary & 0xfff00000) as i32) as i64) >> 20;
                let target = (xregs[rs1] + imm - 4) & !1;
                if target % 4 != 0 {
                    return Err(Exception::InstructionAddressMisaligned(String::from(
                        "must be aligned on a four-byte boundary",
                    )));
                }
                self.pc = target as usize;
            }
            0x6F => {
                // J-type
                // jal
                xregs[rd] = (self.pc as i64) + 4;

                let imm20 = (((binary & 0x80000000) as i32) as i64) >> 31;
                let imm10_1 = ((binary & 0x7fe00000) >> 21) as u64;
                let imm11 = ((binary & 0x100000) >> 20) as u64;
                let imm19_12 = ((binary & 0xff000) >> 12) as u64;
                let offset =
                    ((imm20 << 20) as u64 | (imm19_12 << 12) | (imm11 << 11) | (imm10_1 << 1))
                        as i64;
                let target = (self.pc as i64) + offset - 4;
                if target % 4 != 0 {
                    return Err(Exception::InstructionAddressMisaligned(String::from(
                        "must be aligned on a four-byte boundary",
                    )));
                }
                self.pc = target as usize;
            }
            0x73 => {
                // I-type
                let csr_address = (binary & 0xfff00000) >> 20;
                match funct3 {
                    0x0 => {
                        match (rs2, funct7) {
                            // TODO: implement ecall and ebreak
                            // ecall makes a request of the execution environment by raising an
                            // environment call exception.
                            // ebreak makes a request of the debugger by raising a breakpoint
                            // exception.
                            (0x0, 0x0) => {
                                // ecall
                                match self.mode {
                                    Mode::User => return Err(Exception::EnvironmentCallFromUMode),
                                    Mode::Supervisor => {
                                        return Err(Exception::EnvironmentCallFromSMode)
                                    }
                                    Mode::Machine => {
                                        return Err(Exception::EnvironmentCallFromMMode)
                                    }
                                    _ => {}
                                }
                            }
                            (0x1, 0x0) => {
                                // ebreak
                                return Err(Exception::Breakpoint);
                            }
                            (0x2, 0x0) => {}  // uret
                            (0x2, 0x8) => {}  // sret
                            (0x2, 0x18) => {} // mret
                            (0x5, 0x8) => {}  // wfi
                            (_, 0x9) => {}    // sfence.vma
                            (_, 0x11) => {}   // hfence.bvma
                            (_, 0x51) => {}   // hfence.bvma
                            _ => {}
                        }
                    }
                    0x1 => {
                        // csrrw
                        xregs[rd] = self.csr.read(csr_address)?;
                        self.csr.write(csr_address, xregs[rs1])?;
                    }
                    0x2 => {
                        // csrrs
                        xregs[rd] = self.csr.read(csr_address)?;
                        self.csr.write(csr_address, xregs[rd] | xregs[rs1])?;
                    }
                    0x3 => {
                        // csrrc
                        xregs[rd] = self.csr.read(csr_address)?;
                        self.csr.write(csr_address, xregs[rd] & (!xregs[rs1]))?;
                    }
                    0x5 => {
                        // csrrwi
                        let uimm = rs1 as u64 as i64;
                        xregs[rd] = self.csr.read(csr_address)?;
                        self.csr.write(csr_address, uimm)?;
                    }
                    0x6 => {
                        // csrrsi
                        let uimm = rs1 as u64 as i64;
                        xregs[rd] = self.csr.read(csr_address)?;
                        self.csr.write(csr_address, xregs[rd] | uimm)?;
                    }
                    0x7 => {
                        // csrrci
                        let uimm = rs1 as u64 as i64;
                        xregs[rd] = self.csr.read(csr_address)?;
                        self.csr.write(csr_address, xregs[rd] & (!uimm))?;
                    }
                    _ => {}
                }
            }
            _ => {
                return Err(Exception::IllegalInstruction(String::from(format!(
                    "not implemented opcode {:#x}",
                    opcode
                ))));
            }
        }
        Ok(())
    }
}
