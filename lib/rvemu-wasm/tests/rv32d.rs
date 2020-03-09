#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

use rvemu_core::bus::DRAM_BASE;
use rvemu_core::emulator::Emulator;

wasm_bindgen_test_configure!(run_in_browser);

const DEFAULT_SP: i64 = 1048000 + 0x8000_0000;

#[wasm_bindgen_test]
pub fn fld_rd_offset_rs1() {
    let mut emu = Emulator::new();
    let data = vec![
        0x93, 0x0f, 0x20, 0x00, // addi x31, x0, 2
        0x13, 0x0f, 0x40, 0x00, // addi x30, x0, 4
        0x87, 0xbf, 0x0f, 0x00, // fld f31, 0(x31)
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
    // x0-x31
    let expected_x = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 4, 2,
    ];
    // f0-f30
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
    // f31
    //assert_eq!(0xbf8700400f130020, cpu.fregs.read(31).to_bits());
    assert_eq!(0x0, cpu.fregs.read(31).to_bits());
}

#[wasm_bindgen_test]
pub fn fsd_rs2_offset_rs1() {
    let mut emu = Emulator::new();
    let data = vec![
        0x93, 0x0f, 0x20, 0x00, // addi x31, x0, 2
        0x13, 0x0f, 0x40, 0x00, // addi x30, x0, 4
        0x27, 0xb0, 0xff, 0x01, // fsd f31, 0(x31)
        0x87, 0xbf, 0x0f, 0x00, // fld f31, 0(x31)
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    emu.start();

    // x0-x31
    let expected_x = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 4, 2,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fmaddd_rd_rs1_rs2_rs3() {
    let mut emu = Emulator::new();
    let data = vec![
        0xc3, 0x0f, 0xdf, 0xe3, // fmadd.d f31, f30, f29, f28
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.fregs.write(28, -0.5);
        cpu.fregs.write(29, 4.2);
        cpu.fregs.write(30, 1.2);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5, 4.2, 1.2, 4.54,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fmsubd_rd_rs1_rs2_rs3() {
    let mut emu = Emulator::new();
    let data = vec![
        0xc7, 0x0f, 0xdf, 0xe3, // fmsub.d f31, f30, f29, f28
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.fregs.write(28, -0.5);
        cpu.fregs.write(29, 4.2);
        cpu.fregs.write(30, 1.2);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5, 4.2, 1.2, 5.54,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fnmaddd_rd_rs1_rs2_rs3() {
    let mut emu = Emulator::new();
    let data = vec![
        0xcb, 0x0f, 0xdf, 0xe3, // fnmadd.d f31, f30, f29, f28
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.fregs.write(28, -0.5);
        cpu.fregs.write(29, 4.2);
        cpu.fregs.write(30, 1.2);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5, 4.2, 1.2, -5.54,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fnmsubd_rd_rs1_rs2_rs3() {
    let mut emu = Emulator::new();
    let data = vec![
        0xcf, 0x0f, 0xdf, 0xe3, // fnmsub.d f31, f30, f29, f28
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.fregs.write(28, -0.5);
        cpu.fregs.write(29, 4.2);
        cpu.fregs.write(30, 1.2);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5, 4.2, 1.2, -4.54,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn faddd_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let data = vec![
        0xd3, 0x0f, 0xdf, 0x03, // fadd.d f31, f30, f29
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.fregs.write(29, 4.2);
        cpu.fregs.write(30, 2.5);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.2, 2.5, 6.7,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fsubd_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let data = vec![
        0xd3, 0x0f, 0xdf, 0x0b, // fsub.d f31, f30, f29
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.fregs.write(29, 4.2);
        cpu.fregs.write(30, 2.8);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.2, 2.8, -1.4,
    ];
    for (i, e) in expected.iter().enumerate() {
        // TODO: workaround for floating point precision problem
        assert_eq!(*e, (cpu.fregs.read(i) * 10.0).round() / 10.0);
    }
}

#[wasm_bindgen_test]
pub fn fmuld_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let data = vec![
        0xd3, 0x0f, 0xdf, 0x13, // fmul.d f31, f30, f29
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.fregs.write(29, 4.2);
        cpu.fregs.write(30, -1.2);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.2, -1.2, -5.04,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fdivd_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let data = vec![
        0xd3, 0x0f, 0xdf, 0x1b, // fdiv.d f31, f30, f29
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.fregs.write(29, -1.2);
        cpu.fregs.write(30, 4.2);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.2, 4.2, -3.5,
    ];
    for (i, e) in expected.iter().enumerate() {
        // TODO: workaround for floating point precision problem
        assert_eq!(*e, (cpu.fregs.read(i) * 10.0).round() / 10.0);
    }
}

#[wasm_bindgen_test]
pub fn fsgnjd_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let data = vec![
        0xd3, 0x0f, 0xdf, 0x23, // fsgnj.d f31, f30, f29
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.fregs.write(29, -1.2);
        cpu.fregs.write(30, 4.2);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.2, 4.2, -4.2,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fsgnjnd_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let data = vec![
        0xd3, 0x1f, 0xdf, 0x23, // fsgnjn.d f31, f30, f29
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.fregs.write(29, -1.2);
        cpu.fregs.write(30, 4.2);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.2, 4.2, 4.2,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fsgnjxd_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let data = vec![
        0xd3, 0x2f, 0xdf, 0x23, // fsgnjx.d f31, f30, f29
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.fregs.write(29, -1.2);
        cpu.fregs.write(30, 4.2);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.2, 4.2, -4.2,
    ];
    for (i, e) in expected.iter().enumerate() {
        // TODO: workaround for floating point precision problem
        assert_eq!(*e, (cpu.fregs.read(i) * 10.0).round() / 10.0);
    }
}

#[wasm_bindgen_test]
pub fn fmind_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let data = vec![
        0xd3, 0x0f, 0xdf, 0x2b, // fmin.d f31, f30, f29
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.fregs.write(29, 4.2);
        cpu.fregs.write(30, -1.2);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.2, -1.2, -1.2,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fmaxd_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let data = vec![
        0xd3, 0x1f, 0xdf, 0x2b, // fmax.d f31, f30, f29
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.fregs.write(29, 4.2);
        cpu.fregs.write(30, -1.2);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.2, -1.2, 4.2,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fcvtsd_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let data = vec![
        0xd3, 0x0f, 0x1f, 0x40, // fcvt.s.d f31, f30
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.fregs.write(30, -1.2);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.2, -1.2,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fcvtds_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let data = vec![
        0xd3, 0x0f, 0x0f, 0x42, // fcvt.d.s f31, f30
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.fregs.write(30, -1.2);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.2, -1.2,
    ];
    for (i, e) in expected.iter().enumerate() {
        // TODO: workaround for floating point precision problem
        assert_eq!(*e, (cpu.fregs.read(i) * 10.0).round() / 10.0);
    }
}

#[wasm_bindgen_test]
pub fn fsqrtd_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let data = vec![
        0xd3, 0x0f, 0x0f, 0x5a, // fmax.d f31, f30
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.fregs.write(30, 4.2);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // f0-f31
    let expected = [
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        4.2,
        2.04939015319192,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fled_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let data = vec![
        0xd3, 0x0f, 0xdf, 0xa3, // fle.d f31, f30, f29
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.fregs.write(29, 4.2);
        cpu.fregs.write(30, 4.2);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // x0-x31
    let expected_x = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 1,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.2, 4.2, 0.0,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fltd_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let data = vec![
        0xd3, 0x1f, 0xdf, 0xa3, // flt.d f31, f30, f29
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.fregs.write(29, 4.2);
        cpu.fregs.write(30, -1.2);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // x0-x31
    let expected_x = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 1,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.2, -1.2, 0.0,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn feqd_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let data = vec![
        0xd3, 0x2f, 0xdf, 0xa3, // feq.d f31, f30, f29
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.fregs.write(29, 4.2);
        cpu.fregs.write(30, 4.2);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // x0-x31
    let expected_x = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 1,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.2, 4.2, 0.0,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fcvtwd_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let data = vec![
        0xd3, 0x8f, 0x0f, 0xc2, // fcvt.w.d x31, f31 (rm: 000)
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.fregs.write(31, -4.2);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // x0-x31
    let expected_x = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, -4,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -4.2,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fcvtwud_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let data = vec![
        0xd3, 0x8f, 0x1f, 0xc2, // fcvt.wu.d x31, f31 (rm: 000)
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.fregs.write(31, 4.2);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // x0-x31
    let expected_x = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 4,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.2,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fcvtdw_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let data = vec![
        0xd3, 0x8f, 0x0f, 0xd2, // fcvt.d.w x31, f31 (rm: 000)
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.xregs.write(31, -4);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // x0-x31
    let expected_x = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, -4,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -4.0,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fcvtdwu_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let data = vec![
        0xd3, 0x8f, 0x1f, 0xd2, // fcvt.d.wu x31, f31 (rm: 000)
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.xregs.write(31, 4);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // x0-x31
    let expected_x = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 4,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.0,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fclassd_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let data = vec![
        0xd3, 0x9f, 0x0f, 0xe2, // fclass.d x31, f31
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    {
        let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
        cpu.fregs.write(31, std::f64::INFINITY);
    }

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");

    // x0-x31
    let expected_x = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 7,
    ];
    // f0-f31
    let expected_f = [
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        std::f64::INFINITY,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}
