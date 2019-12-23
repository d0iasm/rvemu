#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate rvemu;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn mul_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        // addi x31, x0, 3
        0x93, 0x0f, 0x30, 0x00,
        // addi x30, x0, -5
        0x13, 0x0f, 0xb0, 0xff,
        // mul x29, x30, x31
        0xb3, 0x0e, 0xff, 0x03,
    ];

    cpu.start(&mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, -5, 3];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn mulh_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        // addi x31, x0, 1
        0x93, 0x0f, 0x10, 0x00,
        // slli x31, x31, 62
        0x93, 0x9f, 0xef, 0x03,
        // addi x30, x0, 1
        0x13, 0x0f, 0x10, 0x00,
        // slli x30, x30, 62
        0x13, 0x1f, 0xef, 0x03,
        // mulh x29, x30, x31
        0xb3, 0x1e, 0xff, 0x03,
    ];

    cpu.start(&mut mem);

    // TODO: use negative values in x30 and x31
    // hex: 0x40000000_00000000 * 0x40000000_00000000 = 0x20000000_00000000_00000000_00000000

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0x1000000000000000, 0x4000000000000000, 0x4000000000000000];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}
