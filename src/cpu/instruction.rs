use crate::log;
use crate::cpu::*;

pub type InstFunc = fn(&mut Cpu, &Code, &mut Vec<u8>);

// addi rd, rs1, imm (0x13): I-type. rd = rs1 + imm
fn addi(cpu: &mut Cpu, code: &Code, _m: &mut Vec<u8>) {
    // TODO: better way to cast u8 to usize?
    let rd: usize = code.rd as usize;
    let rs1: usize = code.rs1 as usize;
    cpu.registers[rd] = cpu.registers[rs1] + code.imm;
}

// add rd, rs1, rs2 (0x33): R-type. rd = rs1 + rs2
fn add(cpu: &mut Cpu, code: &Code, _m: &mut Vec<u8>) {
    let rd: usize = code.rd as usize;
    let rs1: usize = code.rs1 as usize;
    let rs2: usize = code.rs2 as usize;
    cpu.registers[rd] = cpu.registers[rs1] + cpu.registers[rs2];
}

pub fn init_instructions(instructions: &mut [Option<InstFunc>]) {
    instructions[0x13] = Some(addi);
    instructions[0x33] = Some(add);
}
