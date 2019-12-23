#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate rvemu;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn mulw_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        // addi x31, x0, 2
        0x93, 0x0f, 0x20, 0x00, // addi x30, x0, -2
        0x13, 0x0f, 0xe0, 0xff, // mulw x29, x30, x31
        0xbb, 0x0e, 0xff, 0x03,
    ];

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4,
        -2, 2,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn divw_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        // addi x31, x0, 2
        0x93, 0x0f, 0x20, 0x00, // addi x30, x0, -2
        0x13, 0x0f, 0xe0, 0xff, // divw x29, x30, x31
        0xbb, 0x4e, 0xff, 0x03,
    ];

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1,
        -2, 2,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn divuw_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        // addi x31, x0, 2
        0x93, 0x0f, 0x20, 0x00, // addi x30, x0, 8
        0x13, 0x0f, 0x80, 0x00, // divuw x29, x30, x31
        0xbb, 0x5e, 0xff, 0x03,
    ];

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
        8, 2,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn remw_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        // addi x31, x0, 3
        0x93, 0x0f, 0x30, 0x00, // addi x30, x0, -5
        0x13, 0x0f, 0xb0, 0xff, // remw x29, x30, x31
        0xbb, 0x6e, 0xff, 0x03,
    ];

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -2,
        -5, 3,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn remuw_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        // addi x31, x0, 3
        0x93, 0x0f, 0x30, 0x00, // addi x30, x0, 5
        0x13, 0x0f, 0x50, 0x00, // remuw x29, x30, x31
        0xbb, 0x7e, 0xff, 0x03,
    ];

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
        5, 3,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}
