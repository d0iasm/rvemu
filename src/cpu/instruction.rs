use crate::log;
use crate::cpu::Cpu;

pub type InstFunc = fn(&mut Cpu, &mut Vec<u8>);

// addi rd, rs1, imm (0x13): I-type. rd = rs1 + imm
fn addi(cpu: &mut Cpu, _m: &mut Vec<u8>) {
}

// add rd, rs1, rs2 (0x33): R-type. rd = rs1 + rs2
fn add(cpu: &mut Cpu, _m: &mut Vec<u8>) {

}

pub fn init_instructions(instructions: &mut [Option<InstFunc>]) {
    instructions[0x13] = Some(addi);
    instructions[0x33] = Some(add);
}
