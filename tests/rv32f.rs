#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate rvemu;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn flw_rd_offset_rs1() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        0x93, 0x0f, 0x20, 0x00, // addi x31, x0, 2
        0x13, 0x0f, 0x40, 0x00, // addi x30, x0, 4
        0x87, 0xaf, 0x0f, 0x00, // flw f31, 0(x31)
    ];

    cpu.start(&mut mem);

    // x0-x31
    let expected_x = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        4, 2,
    ];
    // f0-f30
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
    // f31
    assert_eq!(0x0f130020, cpu.fregs[31].to_bits());
}

#[wasm_bindgen_test]
pub fn fsw_rs2_offset_rs1() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        0x93, 0x0f, 0x20, 0x00, // addi x31, x0, 2
        0x13, 0x0f, 0x40, 0x00, // addi x30, x0, 4
        0x27, 0xa0, 0xff, 0x01, // fsw f31, 0(x31)
        0x87, 0xaf, 0x0f, 0x00, // flw f31, 0(x31)
    ];

    cpu.start(&mut mem);

    // x0-x31
    let expected_x = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        4, 2,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
}
