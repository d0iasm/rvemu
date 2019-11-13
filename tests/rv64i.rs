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
        // slliw x32, x31, 3
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
        // srliw x32, x31, 1
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
        // sraiw x32, x31, 1
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

