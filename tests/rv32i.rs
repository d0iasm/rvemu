#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate rvemu;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn lb_rd_offset_rs1() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        0x13, 0x08, 0x50, 0x00,
        0x93, 0x08, 0x30, 0x00,
        0x03, 0x09, 0x20, 0x00,
    ];

    // addi x16, x0, 5
    let bin1 = 0x00500813;
    cpu.execute(bin1, &mut mem);

    // addi x17, x0, 3
    let bin2 = 0x00300893;
    cpu.execute(bin2, &mut mem);

    // lb x18, 4(x0)
    let bin3 = 0x00400903;
    cpu.execute(bin3, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        5, 3, -109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn lh_rd_offset_rs1() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        0x13, 0x08, 0x50, 0x00,
        0x93, 0x08, 0x30, 0x00,
        0x03, 0x19, 0x40, 0x00,
    ];

    // addi x16, x0, 5
    let bin1 = 0x00500813;
    cpu.execute(bin1, &mut mem);

    // addi x17, x0, 3
    let bin2 = 0x00300893;
    cpu.execute(bin2, &mut mem);

    // lh x18, 4(x0)
    let bin3 = 0x00401903;
    cpu.execute(bin3, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        5, 3, 2195, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn lw_rd_offset_rs1() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        0x13, 0x08, 0x50, 0x00,
        0x93, 0x08, 0x30, 0x00,
        0x03, 0x19, 0x40, 0x00,
    ];

    // addi x16, x0, 5
    let bin1 = 0x00500813;
    cpu.execute(bin1, &mut mem);

    // addi x17, x0, 3
    let bin2 = 0x00300893;
    cpu.execute(bin2, &mut mem);

    // lw x18, 4(x0)
    let bin3 = 0x00402903;
    cpu.execute(bin3, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        5, 3, 3147923, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn lbu_rd_offset_rs1() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        0x13, 0x08, 0x50, 0x00,
        0x93, 0x08, 0x30, 0x00,
        0x03, 0x19, 0x40, 0x00,
    ];

    // addi x16, x0, 5
    let bin1 = 0x00500813;
    cpu.execute(bin1, &mut mem);

    // addi x17, x0, 3
    let bin2 = 0x00300893;
    cpu.execute(bin2, &mut mem);

    // lbu x18, 4(x0)
    let bin3 = 0x00404903;
    cpu.execute(bin3, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        5, 3, 147, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn lhu_rd_offset_rs1() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        0x13, 0x08, 0x50, 0x00,
        0x93, 0x08, 0x30, 0x00,
        0x03, 0x19, 0x40, 0x00,
    ];

    // addi x16, x0, 5
    let bin1 = 0x00500813;
    cpu.execute(bin1, &mut mem);

    // addi x17, x0, 3
    let bin2 = 0x00300893;
    cpu.execute(bin2, &mut mem);

    // lbu x18, 4(x0)
    let bin3 = 0x00405903;
    cpu.execute(bin3, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        5, 3, 2195, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn addi_rd_rs1_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x2, x0, 4
    let bin = 0x00400113;
    cpu.execute(bin, &mut mem);

    let expected =
        [0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn slli_rd_rs1_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16 x0, 2
    let bin1 = 0x00200813;
    cpu.execute(bin1, &mut mem);

    // slli x17, x16, 3
    let bin2 = 0x00381893;
    cpu.execute(bin2, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        2, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn slti_rd_rs1_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16 x0, -5
    let bin1 = 0xffb00813;
    cpu.execute(bin1, &mut mem);

    // slti x17, x16, -2
    let bin2 = 0xffe82893;
    cpu.execute(bin2, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        -5, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn sltiu_rd_rs1_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, 2
    let bin1 = 0x00200813;
    cpu.execute(bin1, &mut mem);

    // sltiu, x17, x16, 5
    let bin2 = 0x00583893;
    cpu.execute(bin2, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn xori_rd_rs1_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, 3
    let bin1 = 0x00300813;
    cpu.execute(bin1, &mut mem);

    // xori, x17, x16, 6
    let bin2 = 0x00684893;
    cpu.execute(bin2, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        3, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn srai_rd_rs1_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, -8
    let bin1 = 0xff800813;
    cpu.execute(bin1, &mut mem);

    // srai x17, x16, 2
    let bin2 = 0x40285893;
    cpu.execute(bin2, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        -8, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn srli_rd_rs1_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, 8
    let bin1 = 0x00800813;
    cpu.execute(bin1, &mut mem);

    // srli x17, x16, 2
    let bin2 = 0x00285893;
    cpu.execute(bin2, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        8, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn ori_rd_rs1_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, 3
    let bin1 = 0x00300813;
    cpu.execute(bin1, &mut mem);

    // ori, x17, x16, 6
    let bin2 = 0x00686893;
    cpu.execute(bin2, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        3, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn andi_rd_rs1_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, 4
    let bin1 = 0x00400813;
    cpu.execute(bin1, &mut mem);

    // andi, x17, x16, 7
    let bin2 = 0x00787893;
    cpu.execute(bin2, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn auipc_rd_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // auipc x16, 2
    let bin = 0x00002817;
    cpu.execute(bin, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        8192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn sb_rs2_offset_rs1() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        0x13, 0x08, 0x50, 0x00,
        0x93, 0x08, 0x30, 0x00,
        0x23, 0x02, 0x00, 0x01,
        0x03, 0x09, 0x40, 0x00,
    ];

    // addi x16, x0, -5
    let bin1 = 0xffb00813;
    cpu.execute(bin1, &mut mem);

    // addi x17, x0, 3
    let bin2 = 0x00300893;
    cpu.execute(bin2, &mut mem);

    // sb x16, 4(x0)
    let bin3 = 0x01000223;
    cpu.execute(bin3, &mut mem);

    // lb x18, 4(x0)
    let bin4 = 0x00400903;
    cpu.execute(bin4, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        -5, 3, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn sh_rs2_offset_rs1() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        0x13, 0x08, 0x00, 0xc0,
        0x93, 0x08, 0x30, 0x00,
        0x23, 0x12, 0x00, 0x01,
        0x03, 0x19, 0x40, 0x00,
    ];

    // addi x16, x0, -1024
    let bin1 = 0xc0000813;
    cpu.execute(bin1, &mut mem);

    // addi x17, x0, 3
    let bin2 = 0x00300893;
    cpu.execute(bin2, &mut mem);

    // sh x16, 4(x0)
    let bin3 = 0x01001223;
    cpu.execute(bin3, &mut mem);

    // lh x18, 4(x0)
    let bin4 = 0x00401903;
    cpu.execute(bin4, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        -1024, 3, -1024, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn sw_rs2_offset_rs1() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        0x13, 0x08, 0x00, 0x80,
        0x93, 0x08, 0x30, 0x00,
        0x23, 0x22, 0x00, 0x01,
        0x03, 0x29, 0x40, 0x00,
    ];

    // addi x16, x0, -2048
    let bin1 = 0x80000813;
    cpu.execute(bin1, &mut mem);

    // addi x17, x0, 3
    let bin2 = 0x00300893;
    cpu.execute(bin2, &mut mem);

    // sw x16, 4(x0)
    let bin3 = 0x01002223;
    cpu.execute(bin3, &mut mem);

    // lw x18, 4(x0)
    let bin4 = 0x00402903;
    cpu.execute(bin4, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        -2048, 3, -2048, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn add_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x3, x0, 5
    let bin1 = 0x00500193;
    cpu.execute(bin1, &mut mem);

    // addi x4, x0, 6
    let bin2 = 0x00600213;
    cpu.execute(bin2, &mut mem);

    // add x2, x3, x4
    let bin3 = 0x00418133;
    cpu.execute(bin3, &mut mem);

    let expected =
        [0, 0, 11, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn sub_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x3, x0, 5
    let bin1 = 0x00500193;
    cpu.execute(bin1, &mut mem);

    // addi x4, x0, 6
    let bin2 = 0x00600213;
    cpu.execute(bin2, &mut mem);

    // sub x2, x3, x4
    let bin3 = 0x40418133;
    cpu.execute(bin3, &mut mem);

    let expected: [i32; 32] =
        [0, 0, -1, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i] as i32);
    }
}

#[wasm_bindgen_test]
pub fn sll_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, 8
    let bin1 = 0x00800813;
    cpu.execute(bin1, &mut mem);

    // addi x17, x0, 2
    let bin2 = 0x00200893;
    cpu.execute(bin2, &mut mem);

    // sll x18, x16, x17
    let bin3 = 0x01181933;
    cpu.execute(bin3, &mut mem);

    let expected: [i32; 32] =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        8, 2, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i] as i32);
    }
}

#[wasm_bindgen_test]
pub fn slt_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, -8
    let bin1 = 0xff800813;
    cpu.execute(bin1, &mut mem);

    // addi x17, x0, 2
    let bin2 = 0x00200893;
    cpu.execute(bin2, &mut mem);

    // slt x18, x16, x17
    let bin3 = 0x01182933;
    cpu.execute(bin3, &mut mem);

    let expected: [i32; 32] =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        -8, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i] as i32);
    }
}

#[wasm_bindgen_test]
pub fn sltu_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, 8
    let bin1 = 0x00800813;
    cpu.execute(bin1, &mut mem);

    // addi x17, x0, 2
    let bin2 = 0x00200893;
    cpu.execute(bin2, &mut mem);

    // slt x18, x17, x16
    let bin3 = 0x0108b933;
    cpu.execute(bin3, &mut mem);

    let expected: [i32; 32] =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        8, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i] as i32);
    }
}

#[wasm_bindgen_test]
pub fn xor_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, 3
    let bin1 = 0x00300813;
    cpu.execute(bin1, &mut mem);

    // addi x17, x0, 6
    let bin2 = 0x00600893;
    cpu.execute(bin2, &mut mem);

    // xor x18, x16, x17
    let bin3 = 0x01184933;
    cpu.execute(bin3, &mut mem);

    let expected: [i32; 32] =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        3, 6, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i] as i32);
    }
}

#[wasm_bindgen_test]
pub fn srl_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, 16
    let bin1 = 0x01000813;
    cpu.execute(bin1, &mut mem);

    // addi x17, x0, 2
    let bin2 = 0x00200893;
    cpu.execute(bin2, &mut mem);

    // srl x18, x16, x17
    let bin3 = 0x01185933;
    cpu.execute(bin3, &mut mem);

    let expected: [i32; 32] =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        16, 2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i] as i32);
    }
}

#[wasm_bindgen_test]
pub fn sra_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, -16
    let bin1 = 0xff000813;
    cpu.execute(bin1, &mut mem);

    // addi x17, x0, 2
    let bin2 = 0x00200893;
    cpu.execute(bin2, &mut mem);

    // sra x18, x16, x17
    let bin3 = 0x41185933;
    cpu.execute(bin3, &mut mem);

    let expected: [i32; 32] =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        -16, 2, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i] as i32);
    }
}

#[wasm_bindgen_test]
pub fn or_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, 3
    let bin1 = 0x00300813;
    cpu.execute(bin1, &mut mem);

    // addi x17, x0, 5
    let bin2 = 0x00500893;
    cpu.execute(bin2, &mut mem);

    // xor x18, x16, x17
    let bin3 = 0x01186933;
    cpu.execute(bin3, &mut mem);

    let expected: [i32; 32] =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        3, 5, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i] as i32);
    }
}

#[wasm_bindgen_test]
pub fn and_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, 3
    let bin1 = 0x00300813;
    cpu.execute(bin1, &mut mem);

    // addi x17, x0, 5
    let bin2 = 0x00500893;
    cpu.execute(bin2, &mut mem);

    // and x18, x16, x17
    let bin3 = 0x01187933;
    cpu.execute(bin3, &mut mem);

    let expected: [i32; 32] =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        3, 5, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i] as i32);
    }
}

#[wasm_bindgen_test]
pub fn lui_rd_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // lui x16, 2
    let bin = 0x00002837;
    cpu.execute(bin, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        8192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn beq_rs1_rs2_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, 3
    let bin1 = 0x00300813;
    cpu.execute(bin1, &mut mem);
    cpu.pc += 4;

    // addi x17, x0, 3
    let bin2 = 0x00300893;
    cpu.execute(bin2, &mut mem);
    cpu.pc += 4;

    // beq x16, x17, -8
    let bin3 = 0xff180ce3;
    cpu.execute(bin3, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
    assert_eq!(0, cpu.pc);
}

#[wasm_bindgen_test]
pub fn bne_rs1_rs2_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, 3
    let bin1 = 0x00300813;
    cpu.execute(bin1, &mut mem);
    cpu.pc += 4;

    // addi x17, x0, 5
    let bin2 = 0x00500893;
    cpu.execute(bin2, &mut mem);
    cpu.pc += 4;

    // bne x16, x17, -8
    let bin3 = 0xff181ce3;
    cpu.execute(bin3, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        3, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
    assert_eq!(0, cpu.pc);
}

#[wasm_bindgen_test]
pub fn blt_rs1_rs2_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, -3
    let bin1 = 0xffd00813;
    cpu.execute(bin1, &mut mem);
    cpu.pc += 4;

    // addi x17, x0, 5
    let bin2 = 0x00500893;
    cpu.execute(bin2, &mut mem);
    cpu.pc += 4;

    // blt x16, x17, -8
    let bin3 = 0xff184ce3;
    cpu.execute(bin3, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        -3, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
    assert_eq!(0, cpu.pc);
}

#[wasm_bindgen_test]
pub fn bge_rs1_rs2_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, -3
    let bin1 = 0xffd00813;
    cpu.execute(bin1, &mut mem);
    cpu.pc += 4;

    // addi x17, x0, -3
    let bin2 = 0xffd00893;
    cpu.execute(bin2, &mut mem);
    cpu.pc += 4;

    // bge x16, x17, -4
    let bin3 = 0xff185ee3;
    cpu.execute(bin3, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        -3, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
    assert_eq!(0, cpu.pc);
}

#[wasm_bindgen_test]
pub fn bltu_rs1_rs2_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, 3
    let bin1 = 0x00300813;
    cpu.execute(bin1, &mut mem);
    cpu.pc += 4;

    // addi x17, x0, 5
    let bin2 = 0x00500893;
    cpu.execute(bin2, &mut mem);
    cpu.pc += 4;

    // bltu x16, x17, -8
    let bin3 = 0xff186ce3;
    cpu.execute(bin3, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        3, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
    assert_eq!(0, cpu.pc);
}

#[wasm_bindgen_test]
pub fn bgeu_rs1_rs2_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, 5
    let bin1 = 0x00500813;
    cpu.execute(bin1, &mut mem);
    cpu.pc += 4;

    // addi x17, x0, 3
    let bin2 = 0x00300893;
    cpu.execute(bin2, &mut mem);
    cpu.pc += 4;

    // bgeu x16, x17, -8
    let bin3 = 0xff187ce3;
    cpu.execute(bin3, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        5, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
    assert_eq!(0, cpu.pc);
}

#[wasm_bindgen_test]
pub fn jalr_rd_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, 3
    let bin1 = 0x00300813;
    cpu.execute(bin1, &mut mem);
    cpu.pc += 4;

    // addi x17, x0, 5
    let bin2 = 0x00500893;
    cpu.execute(bin2, &mut mem);
    cpu.pc += 4;

    // jalr x18, x0, 4
    let bin3 = 0x00400967;
    cpu.execute(bin3, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        3, 5, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
    assert_eq!(4, cpu.pc);
}

#[wasm_bindgen_test]
pub fn jal_rd_imm() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = Vec::new();

    // addi x16, x0, 3
    let bin1 = 0x00300813;
    cpu.execute(bin1, &mut mem);
    cpu.pc += 4;

    // addi x17, x0, 5
    let bin2 = 0x00500893;
    cpu.execute(bin2, &mut mem);
    cpu.pc += 4;

    // jal x18, -8
    let bin3 = 0xffdff96f;
    cpu.execute(bin3, &mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        3, 5, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
    assert_eq!(0, cpu.pc);
}
