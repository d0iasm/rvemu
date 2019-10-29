use crate::Emulator;

type InstFunc = fn(&mut Emulator);
type Insts = [InstFunc; 256];

pub fn nop(emu: &mut Emulator) {
    emu.pc += 1;
}

pub fn undefined(_emu: &mut Emulator) {
}

pub fn init_instructions(instructions: &mut Insts) {
    instructions[0x01] = nop;
}
