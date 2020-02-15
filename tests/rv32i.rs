#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn lb_rd_offset_rs1() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x50, 0x00, // addi x16, x0, 5
            0x93, 0x08, 0x30, 0x00, // addi x17, x0, 3
            0x03, 0x09, 0x40, 0x00, // lb x18, 4(x0)
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 3, -109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn lh_rd_offset_rs1() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x50, 0x00, // addi x16, x0, 5
            0x93, 0x08, 0x30, 0x00, // addi x17, x0, 3
            0x03, 0x19, 0x40, 0x00, // lh x18, 4(x0)
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 3, 2195, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn lw_rd_offset_rs1() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x50, 0x00, // addi x16, x0, 5
            0x93, 0x08, 0x30, 0x00, // addi x17, x0, 3
            0x03, 0x29, 0x40, 0x00, // lw x18, 4(x0)
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 3, 3147923, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn lbu_rd_offset_rs1() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x50, 0x00, // addi x16, x0, 5
            0x93, 0x08, 0x30, 0x00, // addi x17, x0, 3
            0x03, 0x49, 0x40, 0x00, // lbu x18, 4(x0)
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 3, 147, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn lhu_rd_offset_rs1() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x50, 0x00, // addi x16, x0, 5
            0x93, 0x08, 0x30, 0x00, // addi x17, x0, 3
            0x03, 0x59, 0x40, 0x00, // lbu x18, 4(x0)
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 3, 2195, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn addi_rd_rs1_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x01, 0x40, 0x00, // addi x2, x0, 4
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn slli_rd_rs1_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x20, 0x00, // addi x16 x0, 2
            0x93, 0x18, 0x38, 0x00, // slli x17, x16, 3
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn slti_rd_rs1_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0xb0, 0xff, // addi x16 x0, -5
            0x93, 0x28, 0xe8, 0xff, // slti x17, x16, -2
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn sltiu_rd_rs1_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x20, 0x00, // addi x16, x0, 2
            0x93, 0x38, 0x58, 0x00, // sltiu, x17, x16, 5
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn xori_rd_rs1_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
            0x93, 0x48, 0x68, 0x00, // xori, x17, x16, 6
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn srai_rd_rs1_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x80, 0xff, // addi x16, x0, -8
            0x93, 0x58, 0x28, 0x40, // srai x17, x16, 2
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn srli_rd_rs1_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x80, 0x00, // addi x16, x0, 8
            0x93, 0x58, 0x28, 0x00, // srli x17, x16, 2
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn ori_rd_rs1_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
            0x93, 0x68, 0x68, 0x00, // ori, x17, x16, 6
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn andi_rd_rs1_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x40, 0x00, // addi x16, x0, 4
            0x93, 0x78, 0x78, 0x00, // andi, x17, x16, 7
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn auipc_rd_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x17, 0x28, 0x00, 0x00, // auipc x16, 2
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn sb_rs2_offset_rs1() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0xb0, 0xff, // addi x16, x0, -5
            0x93, 0x08, 0x30, 0x00, // addi x17, x0, 3
            0x23, 0x02, 0x00, 0x01, // sb x16, 4(x0)
            0x03, 0x09, 0x40, 0x00, // lb x18, 4(x0)
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 3, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn sh_rs2_offset_rs1() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x00, 0xc0, // addi x16, x0, -1024
            0x93, 0x08, 0x30, 0x00, // addi x17, x0, 3
            0x23, 0x12, 0x00, 0x01, // sh x16, 4(x0)
            0x03, 0x19, 0x40, 0x00, // lh x18, 4(x0)
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1024, 3, -1024, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn sw_rs2_offset_rs1() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x00, 0x80, // addi x16, x0, -2048
            0x93, 0x08, 0x30, 0x00, // addi x17, x0, 3
            0x23, 0x22, 0x00, 0x01, // sw x16, 4(x0)
            0x03, 0x29, 0x40, 0x00, // lw x18, 4(x0)
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -2048, 3, -2048, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn add_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x93, 0x01, 0x50, 0x00, // addi x3, x0, 5
            0x13, 0x02, 0x60, 0x00, // addi x4, x0, 6
            0x33, 0x81, 0x41, 0x00, // add x2, x3, x4
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 11, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn sub_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x93, 0x01, 0x50, 0x00, // addi x3, x0, 5
            0x13, 0x02, 0x60, 0x00, // addi x4, x0, 6
            0x33, 0x81, 0x41, 0x40, // sub x2, x3, x4
        ],
    };

    cpu.start(&mut mem);

    let expected: [i32; 32] = [
        0, 0, -1, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i] as i32);
    }
}

#[wasm_bindgen_test]
pub fn sll_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x80, 0x00, // addi x16, x0, 8
            0x93, 0x08, 0x20, 0x00, // addi x17, x0, 2
            0x33, 0x19, 0x18, 0x01, // sll x18, x16, x17
        ],
    };

    cpu.start(&mut mem);

    let expected: [i32; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 2, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i] as i32);
    }
}

#[wasm_bindgen_test]
pub fn slt_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x80, 0xff, // addi x16, x0, -8
            0x93, 0x08, 0x20, 0x00, // addi x17, x0, 2
            0x33, 0x29, 0x18, 0x01, // slt x18, x16, x17
        ],
    };

    cpu.start(&mut mem);

    let expected: [i32; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i] as i32);
    }
}

#[wasm_bindgen_test]
pub fn sltu_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x80, 0x00, // addi x16, x0, 8
            0x93, 0x08, 0x20, 0x00, // addi x17, x0, 2
            0x33, 0xb9, 0x08, 0x01, // slt x18, x17, x16
        ],
    };

    cpu.start(&mut mem);

    let expected: [i32; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i] as i32);
    }
}

#[wasm_bindgen_test]
pub fn xor_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
            0x93, 0x08, 0x60, 0x00, // addi x17, x0, 6
            0x33, 0x49, 0x18, 0x01, // xor x18, x16, x17
        ],
    };

    cpu.start(&mut mem);

    let expected: [i32; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 6, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i] as i32);
    }
}

#[wasm_bindgen_test]
pub fn srl_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x00, 0x01, // addi x16, x0, 16
            0x93, 0x08, 0x20, 0x00, // addi x17, x0, 2
            0x33, 0x59, 0x18, 0x01, // srl x18, x16, x17
        ],
    };

    cpu.start(&mut mem);

    let expected: [i32; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i] as i32);
    }
}

#[wasm_bindgen_test]
pub fn sra_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x00, 0xff, // addi x16, x0, -16
            0x93, 0x08, 0x20, 0x00, // addi x17, x0, 2
            0x33, 0x59, 0x18, 0x41, // sra x18, x16, x17
        ],
    };

    cpu.start(&mut mem);

    let expected: [i32; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 2, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i] as i32);
    }
}

#[wasm_bindgen_test]
pub fn or_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
            0x93, 0x08, 0x50, 0x00, // addi x17, x0, 5
            0x33, 0x69, 0x18, 0x01, // xor x18, x16, x17
        ],
    };

    cpu.start(&mut mem);

    let expected: [i32; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 5, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i] as i32);
    }
}

#[wasm_bindgen_test]
pub fn and_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
            0x93, 0x08, 0x50, 0x00, // addi x17, x0, 5
            0x33, 0x79, 0x18, 0x01, // and x18, x16, x17
        ],
    };

    cpu.start(&mut mem);

    let expected: [i32; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 5, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i] as i32);
    }
}

#[wasm_bindgen_test]
pub fn lui_rd_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x37, 0x28, 0x00, 0x00, // lui x16, 2
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn beq_rs1_rs2_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
            0x93, 0x08, 0x30, 0x00, // addi x17, x0, 3
            0x63, 0x06, 0x18, 0x01, // beq x16, x17, 12
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
    assert_eq!(20, cpu.pc);
}

#[wasm_bindgen_test]
pub fn bne_rs1_rs2_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
            0x93, 0x08, 0x50, 0x00, // addi x17, x0, 5
            0x63, 0x16, 0x18, 0x01, // bne x16, x17, 12
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
    assert_eq!(20, cpu.pc);
}

#[wasm_bindgen_test]
pub fn blt_rs1_rs2_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0xd0, 0xff, // addi x16, x0, -3
            0x93, 0x08, 0x50, 0x00, // addi x17, x0, 5
            0x63, 0x46, 0x18, 0x01, // blt x16, x17, -8
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -3, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
    assert_eq!(20, cpu.pc);
}

#[wasm_bindgen_test]
pub fn bge_rs1_rs2_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0xd0, 0xff, // addi x16, x0, -3
            0x93, 0x08, 0xd0, 0xff, // addi x17, x0, -3
            0x63, 0x56, 0x18, 0x01, // bge x16, x17, 12
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -3, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
    assert_eq!(20, cpu.pc);
}

#[wasm_bindgen_test]
pub fn bltu_rs1_rs2_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
            0x93, 0x08, 0x50, 0x00, // addi x17, x0, 5
            0x63, 0x66, 0x18, 0x01, // bltu x16, x17, 12
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
    assert_eq!(20, cpu.pc);
}

#[wasm_bindgen_test]
pub fn bgeu_rs1_rs2_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x50, 0x00, // addi x16, x0, 5
            0x93, 0x08, 0x30, 0x00, // addi x17, x0, 3
            0x63, 0x76, 0x18, 0x01, // bgeu x16, x17, 12
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
    assert_eq!(20, cpu.pc);
}

#[wasm_bindgen_test]
pub fn jalr_rd_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
            0x93, 0x08, 0x50, 0x00, // addi x17, x0, 5
            0x67, 0x09, 0xa0, 0x02, // jalr x18, x0, 42
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 5, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
    assert_eq!(42, cpu.pc);
}

#[wasm_bindgen_test]
pub fn jal_rd_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
            0x93, 0x08, 0x50, 0x00, // addi x17, x0, 5
            0x6f, 0x09, 0xc0, 0x00, // jal x18, 12
        ],
    };

    cpu.start(&mut mem);

    let expected = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 5, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
    assert_eq!(20, cpu.pc);
}
