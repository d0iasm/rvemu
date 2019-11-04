/*
#[cfg(test)]
mod tests {
    #[test]
    fn add_rd_rs1_rs2() {
        let mut cpu = riscv_emu::cpu::Cpu::new();
        // addi x2, x0, 4
        let bin = 0x00310133;
        let mut mem = Vec::new();
        cpu.execute(bin, &mut mem);
        assert_eq!(0, cpu.regs[0]);
        assert_eq!(0, cpu.regs[1]);
        //assert_eq!(4, cpu.regs[2]);
    }
}
*/
#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate riscv_emu;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn addi_rd_rs1_imm() {
    let mut cpu = riscv_emu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x2, x0, 4
    let bin = 0x00400113;
    cpu.execute(bin, &mut mem);

    let expected =
        [0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn add_rd_rs1_rs2() {
    let mut cpu = riscv_emu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x3, x0, 5
    let bin1 = 0x00500193;
    cpu.execute(bin1, &mut mem);

    // addi x4, x0, 6
    let bin2 = 0x00600213;
    cpu.execute(bin2, &mut mem);

    // add x2, x3, x4
    let bin3 = 0x00418133;
    cpu.execute(bin3, &mut mem);

    let expected =
        [0, 0, 11, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn sub_rd_rs1_rs2() {
    let mut cpu = riscv_emu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x3, x0, 5
    let bin1 = 0x00500193;
    cpu.execute(bin1, &mut mem);

    // addi x4, x0, 6
    let bin2 = 0x00600213;
    cpu.execute(bin2, &mut mem);

    // sub x2, x3, x4
    let bin3 = 0x40418133;
    cpu.execute(bin3, &mut mem);

    let expected: [i32; 32] =
        [0, 0, -1, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i] as i32);
    }
}
