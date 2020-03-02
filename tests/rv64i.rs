#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const DEFAULT_SP: i64 = 1048000;

#[wasm_bindgen_test]
pub fn ld_rd_offset_rs1() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut mem = rvemu_core::memory::Memory {
        dram: vec![
            0x93, 0x0f, 0x50, 0x00, // addi x31, x0, 5
            0x13, 0x0f, 0x30, 0x00, // addi x30, x0, 3
            0x83, 0x3E, 0x40, 0x00, // ld x29, 4(x0)
        ],
    };

    cpu.start(&mut mem);

    // memory layout
    // 0x0000000c   ...
    // 0x00000008   83  3e  40  00
    // 0x00000004   13  0f  30  00
    // 0x00000000   93  0f  50  00

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
        0x00403e8300300f13,
        3,
        5,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn lwu_rd_offset_rs1() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut mem = rvemu_core::memory::Memory {
        dram: vec![
            0x93, 0x0f, 0xb0, 0xff, // addi x31, x0, -5
            0x13, 0x0f, 0x30, 0x00, // addi x30, x0, 3
            0x83, 0x6E, 0x00, 0x00, // lwu x29, 0(x0)
        ],
    };

    cpu.start(&mut mem);

    // memory layout
    // 0x0000000c   ...
    // 0x00000008   83  6e  00  00
    // 0x00000004   13  0f  30  00
    // 0x00000000   93  0f  b0  ff

    let expected = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0xffb00f93, 3, -5,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn addiw_rd_rs1_imm() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut mem = rvemu_core::memory::Memory {
        dram: vec![
            // addiw x31, x0, 5
            0x9B, 0x0F, 0x50, 0x00,
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 5,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn slliw_rd_rs1_imm() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut mem = rvemu_core::memory::Memory {
        dram: vec![
            0x9B, 0x0F, 0x40, 0x00, // addiw x31, x0, 4
            0x1B, 0x9F, 0x3F, 0x00, // slliw x30, x31, 3
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 32, 4,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn srliw_rd_rs1_imm() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut mem = rvemu_core::memory::Memory {
        dram: vec![
            0x9B, 0x0F, 0x40, 0x00, // addiw x31, x0, 4
            0x1B, 0xDF, 0x1F, 0x00, // srliw x30, x31, 1
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 2, 4,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn sraiw_rd_rs1_imm() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut mem = rvemu_core::memory::Memory {
        dram: vec![
            0x9B, 0x0F, 0xCF, 0xFF, // addiw x31, x0, -4
            0x1B, 0xDF, 0x1F, 0x40, // sraiw x30, x31, 1
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, -2, -4,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn addw_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut mem = rvemu_core::memory::Memory {
        dram: vec![
            0x9B, 0x0F, 0xCF, 0xFF, // addiw x31, x0, -4
            0x1B, 0x0F, 0x80, 0x00, // addiw x30, x0, 8
            0xBB, 0x0E, 0xFF, 0x01, // addw x29, x30, x31
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 4, 8, -4,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn subw_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut mem = rvemu_core::memory::Memory {
        dram: vec![
            0x9B, 0x0F, 0xCF, 0xFF, // addiw x31, x0, -4
            0x1B, 0x0F, 0x80, 0x00, // addiw x30, x0, 8
            0xBB, 0x0E, 0xFF, 0x41, // subw x29, x30, x31
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 12, 8, -4,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn sllw_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut mem = rvemu_core::memory::Memory {
        dram: vec![
            0x9B, 0x0F, 0x20, 0x00, // addiw x31, x0, 2
            0x1B, 0x0F, 0x80, 0x00, // addiw x30, x0, 8
            0xBB, 0x1E, 0xFF, 0x01, // sllw x29, x30, x31
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 32, 8, 2,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn srlw_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut mem = rvemu_core::memory::Memory {
        dram: vec![
            0x9B, 0x0F, 0x20, 0x00, // addiw x31, x0, 2
            0x1B, 0x0F, 0x80, 0x00, // addiw x30, x0, 8
            0xBB, 0x5E, 0xFF, 0x01, // srlw x29, x30, x31
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 2, 8, 2,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn sraw_rd_rs1_rs2() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut mem = rvemu_core::memory::Memory {
        dram: vec![
            0x9B, 0x0F, 0x20, 0x00, // addiw x31, x0, 2
            0x1B, 0x0F, 0x8F, 0xFF, // addiw x30, x0, -8
            0xBB, 0x5E, 0xFF, 0x41, // sraw x29, x30, x31
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, -2, -8, 2,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn sd_rs2_offset_rs1() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut mem = rvemu_core::memory::Memory {
        dram: vec![
            0x93, 0x0f, 0xb0, 0xff, // addi x31, x0, -5
            0x13, 0x0f, 0x30, 0x00, // addi x30, x0, 3
            0x23, 0x30, 0xf0, 0x01, // sw x31, 0(x0)
            0x83, 0x3E, 0x00, 0x00, // ld x29, 0(x0)
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, -5, 3, -5,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}
