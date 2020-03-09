#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

use rvemu_core::bus::DRAM_BASE;

wasm_bindgen_test_configure!(run_in_browser);

const DEFAULT_SP: i64 = 1048000 + 0x8000_0000;

#[wasm_bindgen_test]
pub fn mul_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0x93, 0x0f, 0x30, 0x00, // addi x31, x0, 3
        0x13, 0x0f, 0xb0, 0xff, // addi x30, x0, -5
        0xb3, 0x0e, 0xff, 0x03, // mul x29, x30, x31
    ];

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

    let expected = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, -15, -5, 3,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn mulh_rd_rs1_rs2() {
    // TODO: make sure `mulh` and `mulhsu` works correctly
    let mut emu = Emulator::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0x93, 0x0f, 0x10, 0x00, // addi x31, x0, 1
        0x93, 0x9f, 0xef, 0x03, // slli x31, x31, 62
        0x13, 0x0f, 0x10, 0x00, // addi x30, x0, 1
        0x13, 0x1f, 0xef, 0x03, // slli x30, x30, 62
        0xb3, 0x1e, 0xff, 0x03, // mulh x29, x30, x31
    ];

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

    // TODO: use negative values in x30 and x31
    // hex: 0x40000000_00000000 * 0x40000000_00000000 = 0x20000000_00000000_00000000_00000000

    let expected = [
        0,
        0,
        DEFAULT_SP,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0x1000000000000000,
        0x4000000000000000,
        0x4000000000000000,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn mulhsu_rd_rs1_rs2() {
    // TODO: make sure `mulh` and `mulhsu` works correctly
    let mut emu = Emulator::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0x93, 0x0f, 0x10, 0x00, // addi x31, x0, 1
        0x93, 0x9f, 0xef, 0x03, // slli x31, x31, 62
        0x13, 0x0f, 0x10, 0x00, // addi x30, x0, 1
        0x13, 0x1f, 0xef, 0x03, // slli x30, x30, 62
        0xb3, 0x2e, 0xff, 0x03, // mulhsu x29, x30, x31
    ];

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

    // TODO: use a negative value for x30
    // hex: 0x40000000_00000000 * 0x40000000_00000000 = 0x20000000_00000000_00000000_00000000

    let expected = [
        0,
        0,
        DEFAULT_SP,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0x1000000000000000,
        0x4000000000000000,
        0x4000000000000000,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn mulhu_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0x93, 0x0f, 0x10, 0x00, // addi x31, x0, 1
        0x93, 0x9f, 0xef, 0x03, // slli x31, x31, 62
        0x13, 0x0f, 0x10, 0x00, // addi x30, x0, 1
        0x13, 0x1f, 0xef, 0x03, // slli x30, x30, 62
        0xb3, 0x3e, 0xff, 0x03, // mulhu x29, x30, x31
    ];

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

    // hex: 0x40000000_00000000 * 0x40000000_00000000 = 0x10000000_00000000_00000000_00000000

    let expected = [
        0,
        0,
        DEFAULT_SP,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0x1000000000000000,
        0x4000000000000000,
        0x4000000000000000,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn div_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0x93, 0x0f, 0x30, 0x00, // addi x31, x0, 3
        0x13, 0x0f, 0xb0, 0xff, // addi x30, x0, -5
        0xb3, 0x4e, 0xff, 0x03, // div x29, x30, x31
    ];

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

    let expected = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, -1, -5, 3,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn divu_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0x93, 0x0f, 0x30, 0x00, // addi x31, x0, 3
        0x13, 0x0f, 0x50, 0x00, // addi x30, x0, 5
        0xb3, 0x4e, 0xff, 0x03, // div x29, x30, x31
    ];

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

    let expected = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 5, 3,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn rem_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0x93, 0x0f, 0x30, 0x00, // addi x31, x0, 3
        0x13, 0x0f, 0xb0, 0xff, // addi x30, x0, -5
        0xb3, 0x6e, 0xff, 0x03, // rem x29, x30, x31
    ];

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

    let expected = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, -2, -5, 3,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn remu_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0x93, 0x0f, 0x30, 0x00, // addi x31, x0, 3
        0x13, 0x0f, 0x50, 0x00, // addi x30, x0, 5
        0xb3, 0x7e, 0xff, 0x03, // remu x29, x30, x31
    ];

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

    let expected = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 2, 5, 3,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}
