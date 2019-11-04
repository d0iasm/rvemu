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
pub fn slli_rd_rs1_imm() {
    let mut cpu = riscv_emu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16 x0, 2
    let bin1 = 0x00200813;
    cpu.execute(bin1, &mut mem);

    // slli x17, x16, 3
    let bin2 = 0x00381893;
    cpu.execute(bin2, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        2, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn slti_rd_rs1_imm() {
    let mut cpu = riscv_emu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16 x0, -5
    let bin1 = 0xffb00813;
    cpu.execute(bin1, &mut mem);

    // slti x17, x16, -2
    let bin2 = 0xffe82893;
    cpu.execute(bin2, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        -5, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn sltiu_rd_rs1_imm() {
    let mut cpu = riscv_emu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, 2
    let bin1 = 0x00200813;
    cpu.execute(bin1, &mut mem);

    // sltiu, x17, x16, 5
    let bin2 = 0x00583893;
    cpu.execute(bin2, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn xori_rd_rs1_imm() {
    let mut cpu = riscv_emu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, 3
    let bin1 = 0x00300813;
    cpu.execute(bin1, &mut mem);

    // xori, x17, x16, 6
    let bin2 = 0x00684893;
    cpu.execute(bin2, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        3, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn srli_rd_rs1_imm() {
    let mut cpu = riscv_emu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, -8
    let bin1 = 0xff800813;
    cpu.execute(bin1, &mut mem);

    // srai x17, x16, 2
    let bin2 = 0x40285893;
    cpu.execute(bin2, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        -8, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn srai_rd_rs1_imm() {
    let mut cpu = riscv_emu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, 8
    let bin1 = 0x00800813;
    cpu.execute(bin1, &mut mem);

    // srli x17, x16, 2
    let bin2 = 0x00285893;
    cpu.execute(bin2, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        8, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn ori_rd_rs1_imm() {
    let mut cpu = riscv_emu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, 3
    let bin1 = 0x00300813;
    cpu.execute(bin1, &mut mem);

    // ori, x17, x16, 6
    let bin2 = 0x00686893;
    cpu.execute(bin2, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        3, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn andi_rd_rs1_imm() {
    let mut cpu = riscv_emu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, 4
    let bin1 = 0x00400813;
    cpu.execute(bin1, &mut mem);

    // andi, x17, x16, 7
    let bin2 = 0x00787893;
    cpu.execute(bin2, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
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
