#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate rvemu;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn addiw_rd_rs1_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        // addiw x31, x0, 5
        0x9B, 0x0F, 0x50, 0x00,
    ];

    cpu.start(&mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn slliw_rd_rs1_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        // addiw x31, x0, 4
        0x9B, 0x0F, 0x40, 0x00,
        // slliw x30, x31, 3
        0x1B, 0x9F, 0x3F, 0x00,
    ];

    cpu.start(&mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 4];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn srliw_rd_rs1_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        // addiw x31, x0, 4
        0x9B, 0x0F, 0x40, 0x00,
        // srliw x30, x31, 1
        0x1B, 0xDF, 0x1F, 0x00,
    ];

    cpu.start(&mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 4];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn sraiw_rd_rs1_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        // addiw x31, x0, -4
        0x9B, 0x0F, 0xCF, 0xFF,
        // sraiw x30, x31, 1
        0x1B, 0xDF, 0x1F, 0x40,
    ];

    cpu.start(&mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -2, -4];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn addw_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        // addiw x31, x0, -4
        0x9B, 0x0F, 0xCF, 0xFF,
        // addiw x30, x0, 8
        0x1B, 0x0F, 0x80, 0x00,
        // addw x29, x30, x31
        0xBB, 0x0E, 0xFF, 0x01,
    ];

    cpu.start(&mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 8, -4];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn subw_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        // addiw x31, x0, -4
        0x9B, 0x0F, 0xCF, 0xFF,
        // addiw x30, x0, 8
        0x1B, 0x0F, 0x80, 0x00,
        // subw x29, x30, x31
        0xBB, 0x0E, 0xFF, 0x41,
    ];

    cpu.start(&mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 8, -4];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn sllw_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        // addiw x31, x0, 2
        0x9B, 0x0F, 0x20, 0x00,
        // addiw x30, x0, 8
        0x1B, 0x0F, 0x80, 0x00,
        // sllw x29, x30, x31
        0xBB, 0x1E, 0xFF, 0x01,
    ];

    cpu.start(&mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 8, 2];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn srlw_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        // addiw x31, x0, 2
        0x9B, 0x0F, 0x20, 0x00,
        // addiw x30, x0, 8
        0x1B, 0x0F, 0x80, 0x00,
        // srlw x29, x30, x31
        0xBB, 0x5E, 0xFF, 0x01,
    ];

    cpu.start(&mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 8, 2];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn sraw_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        // addiw x31, x0, 2
        0x9B, 0x0F, 0x20, 0x00,
        // addiw x30, x0, -8
        0x1B, 0x0F, 0x8F, 0xFF,
        // sraw x29, x30, x31
        0xBB, 0x5E, 0xFF, 0x41,
    ];

    cpu.start(&mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -2, -8, 2];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

