#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

use rvemu_core::bus::DRAM_BASE;

wasm_bindgen_test_configure!(run_in_browser);

const DEFAULT_SP: i64 = 1048000 + 0x8000_0000;

#[wasm_bindgen_test]
pub fn flw_rd_offset_rs1() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0x93, 0x0f, 0x20, 0x00, // addi x31, x0, 2
        0x13, 0x0f, 0x40, 0x00, // addi x30, x0, 4
        0x87, 0xaf, 0x0f, 0x00, // flw f31, 0(x31)
    ];
    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

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
    //assert_eq!(0x0f130020, cpu.fregs.read(31).to_bits());
    assert_eq!(0x0, cpu.fregs.read(31).to_bits());
}

#[wasm_bindgen_test]
pub fn fsw_rs2_offset_rs1() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0x93, 0x0f, 0x20, 0x00, // addi x31, x0, 2
        0x13, 0x0f, 0x40, 0x00, // addi x30, x0, 4
        0x27, 0xa0, 0xff, 0x01, // fsw f31, 0(x31)
        0x87, 0xaf, 0x0f, 0x00, // flw f31, 0(x31)
    ];
    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

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
pub fn fmadds_rd_rs1_rs2_rs3() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xc3, 0x0f, 0xdf, 0xe1, // fmadd.s f31, f30, f29, f28
    ];

    cpu.fregs.write(28, -0.5);
    cpu.fregs.write(29, 4.2);
    cpu.fregs.write(30, 1.2);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5, 4.2, 1.2, 4.54,
    ];
    for (i, e) in expected.iter().enumerate() {
        // TODO: workaround for floating point precision problem
        assert_eq!(*e, (cpu.fregs.read(i) * 100.0).round() / 100.0);
    }
}

#[wasm_bindgen_test]
pub fn fmsubs_rd_rs1_rs2_rs3() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xc7, 0x0f, 0xdf, 0xe1, // fmsub.s f31, f30, f29, f28
    ];

    cpu.fregs.write(28, -0.5);
    cpu.fregs.write(29, 4.2);
    cpu.fregs.write(30, 1.2);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5, 4.2, 1.2, 5.54,
    ];
    for (i, e) in expected.iter().enumerate() {
        // TODO: workaround for floating point precision problem
        assert_eq!(*e, (cpu.fregs.read(i) * 100.0).round() / 100.0);
    }
}

#[wasm_bindgen_test]
pub fn fnmadds_rd_rs1_rs2_rs3() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xcb, 0x0f, 0xdf, 0xe1, // fnmadd.s f31, f30, f29, f28
    ];

    cpu.fregs.write(28, -0.5);
    cpu.fregs.write(29, 4.2);
    cpu.fregs.write(30, 1.2);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5, 4.2, 1.2, -5.54,
    ];
    for (i, e) in expected.iter().enumerate() {
        // TODO: workaround for floating point precision problem
        assert_eq!(*e, (cpu.fregs.read(i) * 100.0).round() / 100.0);
    }
}

#[wasm_bindgen_test]
pub fn fnmsubs_rd_rs1_rs2_rs3() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xcf, 0x0f, 0xdf, 0xe1, // fnmsub.s f31, f30, f29, f28
    ];

    cpu.fregs.write(28, -0.5);
    cpu.fregs.write(29, 4.2);
    cpu.fregs.write(30, 1.2);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5, 4.2, 1.2, -4.54,
    ];
    for (i, e) in expected.iter().enumerate() {
        // TODO: workaround for floating point precision problem
        assert_eq!(*e, (cpu.fregs.read(i) * 100.0).round() / 100.0);
    }
}

#[wasm_bindgen_test]
pub fn fadds_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x0f, 0xdf, 0x01, // fadd.s f31, f30, f29
    ];

    cpu.fregs.write(29, 4.2);
    cpu.fregs.write(30, 2.5);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.2, 2.5, 6.7,
    ];
    for (i, e) in expected.iter().enumerate() {
        // TODO: workaround for floating point precision problem
        assert_eq!(*e, (cpu.fregs.read(i) * 10.0).round() / 10.0);
    }
}

#[wasm_bindgen_test]
pub fn fsubs_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x0f, 0xdf, 0x09, // fsub.s f31, f30, f29
    ];

    cpu.fregs.write(29, 4.2);
    cpu.fregs.write(30, 2.8);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

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
pub fn fmuls_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x0f, 0xdf, 0x11, // fmul.s f31, f30, f29
    ];

    cpu.fregs.write(29, 4.2);
    cpu.fregs.write(30, -1.2);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.2, -1.2, -5.04,
    ];
    for (i, e) in expected.iter().enumerate() {
        // TODO: workaround for floating point precision problem
        assert_eq!(*e, (cpu.fregs.read(i) * 100.0).round() / 100.0);
    }
}

#[wasm_bindgen_test]
pub fn fdivs_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x0f, 0xdf, 0x19, // fdiv.s f31, f30, f29
    ];

    cpu.fregs.write(29, -1.2);
    cpu.fregs.write(30, 4.2);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

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
pub fn fsgnjs_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x0f, 0xdf, 0x21, // fsgnj.s f31, f30, f29
    ];

    cpu.fregs.write(29, -1.2);
    cpu.fregs.write(30, 4.2);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

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
pub fn fsgnjns_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x1f, 0xdf, 0x21, // fsgnjn.s f31, f30, f29
    ];

    cpu.fregs.write(29, -1.2);
    cpu.fregs.write(30, 4.2);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

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
pub fn fsgnjxs_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x2f, 0xdf, 0x21, // fsgnjx.s f31, f30, f29
    ];

    cpu.fregs.write(29, -1.2);
    cpu.fregs.write(30, 4.2);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

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
pub fn fmins_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x0f, 0xdf, 0x29, // fmin.s f31, f30, f29
    ];

    cpu.fregs.write(29, 4.2);
    cpu.fregs.write(30, -1.2);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

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
pub fn fmaxs_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x1f, 0xdf, 0x29, // fmax.s f31, f30, f29
    ];

    cpu.fregs.write(29, 4.2);
    cpu.fregs.write(30, -1.2);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

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
pub fn fsqrts_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x0f, 0x0f, 0x58, // fmax.s f31, f30
    ];

    cpu.fregs.write(30, 4.2);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

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
        2.0493900775909424,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fles_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x0f, 0xdf, 0xa1, // fle.s f31, f30, f29
    ];

    cpu.fregs.write(29, 4.2);
    cpu.fregs.write(30, 4.2);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

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
pub fn flts_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x1f, 0xdf, 0xa1, // flt.s f31, f30, f29
    ];

    cpu.fregs.write(29, 4.2);
    cpu.fregs.write(30, -1.2);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

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
pub fn feqs_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x2f, 0xdf, 0xa1, // feq.s f31, f30, f29
    ];

    cpu.fregs.write(29, 4.2);
    cpu.fregs.write(30, 4.2);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

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
pub fn fcvtws_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x8f, 0x0f, 0xc0, // fcvt.w.s x31, f31 (rm: 000)
    ];

    cpu.fregs.write(31, -4.2);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

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
pub fn fcvtwus_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x8f, 0x1f, 0xc0, // fcvt.wu.s x31, f31 (rm: 000)
    ];

    cpu.fregs.write(31, 4.2);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

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
pub fn fcvtsw_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x8f, 0x0f, 0xd0, // fcvt.s.w x31, f31 (rm: 000)
    ];

    cpu.xregs.write(31, -4);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

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
pub fn fcvtswu_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x8f, 0x1f, 0xd0, // fcvt.s.wu x31, f31 (rm: 000)
    ];

    cpu.xregs.write(31, 4);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

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
pub fn fmvxw_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x8f, 0x0f, 0xe0, // fmv.x.w x31, f31
    ];

    cpu.fregs.write(31, 4.0);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

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
pub fn fclasss_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x9f, 0x0f, 0xe0, // fclass.s x31, f31
    ];

    cpu.fregs.write(31, std::f64::INFINITY);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

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

#[wasm_bindgen_test]
pub fn fmvwx_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x8f, 0x0f, 0xf0, // fmv.w.x x31, f31
    ];

    cpu.xregs.write(31, 4);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

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
