use crate::log;
use crate::cpu::Cpu;

pub type InstFunc = fn(&mut Cpu, &mut Vec<u8>);

// add rd, rs1, rs2 (0x33): R-type. rd = rs1 + rs2
fn add(cpu: &mut Cpu, memory: &mut Vec<u8>) {
    log("Called add !!!!!!!!!!!");
}

pub fn init_instructions(instructions: &mut [Option<InstFunc>]) {
    // opcode exists in [0x06-0x00].
    instructions[0x33] = Some(add);
}
