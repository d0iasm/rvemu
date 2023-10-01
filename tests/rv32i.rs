mod helper;

use rvemu::emulator::Emulator;
use rvemu::bus::DRAM_BASE;

#[test]
fn lb_rd_offset_rs1() {
    let mut _emu = Emulator::new();

    let _data = vec![
        0x13, 0x08, 0x50, 0x00, // addi x16, x0, 5
        0x93, 0x08, 0x30, 0x00, // addi x17, x0, 3
        0x03, 0x09, 0x40, 0x00, // lb x18, 4(x0)
    ];
    let _expected_xregs = helper::create_xregs(vec![(16, 5), (17, 3), (18, -109i64 as u64)]);
    let _expected_fregs = helper::create_fregs(vec![]);

    // TODO: fix LoadAccessFault
    //helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn lh_rd_offset_rs1() {
    let mut _emu = Emulator::new();

    let _data = vec![
        0x13, 0x08, 0x50, 0x00, // addi x16, x0, 5
        0x93, 0x08, 0x30, 0x00, // addi x17, x0, 3
        0x03, 0x19, 0x40, 0x00, // lh x18, 4(x0)
    ];
    let _expected_xregs = helper::create_xregs(vec![(16, 5), (17, 3), (18, 0x0893)]);
    let _expected_fregs = helper::create_fregs(vec![]);

    // TODO: fix LoadAccessFault
    //helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn lw_rd_offset_rs1() {
    let mut _emu = Emulator::new();

    let _data = vec![
        0x13, 0x08, 0x50, 0x00, // addi x16, x0, 5
        0x93, 0x08, 0x30, 0x00, // addi x17, x0, 3
        0x03, 0x29, 0x40, 0x00, // lw x18, 4(x0)
    ];
    let _expected_xregs = helper::create_xregs(vec![(16, 5), (17, 3), (18, 0x300893)]);
    let _expected_fregs = helper::create_fregs(vec![]);

    // TODO: fix LoadAccessFault
    //helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn lbu_rd_offset_rs1() {
    let mut _emu = Emulator::new();

    let _data = vec![
        0x13, 0x08, 0x50, 0x00, // addi x16, x0, 5
        0x93, 0x08, 0x30, 0x00, // addi x17, x0, 3
        0x03, 0x49, 0x40, 0x00, // lbu x18, 4(x0)
    ];
    let _expected_xregs = helper::create_xregs(vec![(16, 5), (17, 3), (18, 0x93)]);
    let _expected_fregs = helper::create_fregs(vec![]);

    // TODO: fix LoadAccessFault
    //helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn lhu_rd_offset_rs1() {
    let mut _emu = Emulator::new();

    let _data = vec![
        0x13, 0x08, 0x50, 0x00, // addi x16, x0, 5
        0x93, 0x08, 0x30, 0x00, // addi x17, x0, 3
        0x03, 0x59, 0x40, 0x00, // lbu x18, 4(x0)
    ];
    let _expected_xregs = helper::create_xregs(vec![(16, 5), (17, 3), (18, 0x0893)]);
    let _expected_fregs = helper::create_fregs(vec![]);

    // TODO: fix LoadAccessFault
    //helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn addi_rd_rs1_imm() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x93, 0x0F, 0x40, 0x00, // addi x31, x0, 4
    ];
    let expected_xregs = helper::create_xregs(vec![(31, 4)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn slli_rd_rs1_imm() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0x20, 0x00, // addi x16 x0, 2
        0x93, 0x18, 0x38, 0x00, // slli x17, x16, 3
    ];
    let expected_xregs = helper::create_xregs(vec![(16, 2), (17, 16)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn slti_rd_rs1_imm() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0xb0, 0xff, // addi x16 x0, -5
        0x93, 0x28, 0xe8, 0xff, // slti x17, x16, -2
    ];
    let expected_xregs = helper::create_xregs(vec![(16, -5i64 as u64), (17, 1)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn sltiu_rd_rs1_imm() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0x20, 0x00, // addi x16, x0, 2
        0x93, 0x38, 0x58, 0x00, // sltiu, x17, x16, 5
    ];
    let expected_xregs = helper::create_xregs(vec![(16, 2), (17, 1)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn xori_rd_rs1_imm() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
        0x93, 0x48, 0x68, 0x00, // xori, x17, x16, 6
    ];
    let expected_xregs = helper::create_xregs(vec![(16, 3), (17, 5)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn srai_rd_rs1_imm() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0x80, 0xff, // addi x16, x0, -8
        0x93, 0x58, 0x28, 0x40, // srai x17, x16, 2
    ];
    let expected_xregs = helper::create_xregs(vec![(16, -8i64 as u64), (17, -2i64 as u64)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn srli_rd_rs1_imm() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0x80, 0x00, // addi x16, x0, 8
        0x93, 0x58, 0x28, 0x00, // srli x17, x16, 2
    ];
    let expected_xregs = helper::create_xregs(vec![(16, 8), (17, 2)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn ori_rd_rs1_imm() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
        0x93, 0x68, 0x68, 0x00, // ori, x17, x16, 6
    ];
    let expected_xregs = helper::create_xregs(vec![(16, 3), (17, 7)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn andi_rd_rs1_imm() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0x40, 0x00, // addi x16, x0, 4
        0x93, 0x78, 0x78, 0x00, // andi, x17, x16, 7
    ];
    let expected_xregs = helper::create_xregs(vec![(16, 4), (17, 4)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn auipc_rd_imm() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x17, 0x28, 0x00, 0x00, // auipc x16, 2
    ];
    let expected_xregs = helper::create_xregs(vec![(16, 0x2000 + DRAM_BASE)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn sb_rs2_offset_rs1() {
    let mut _emu = Emulator::new();
    
    let _data = vec![
        0x13, 0x08, 0xb0, 0xff, // addi x16, x0, -5
        0x93, 0x08, 0x30, 0x00, // addi x17, x0, 3
        0x23, 0x02, 0x00, 0x01, // sb x16, 4(x0)
        0x03, 0x09, 0x40, 0x00, // lb x18, 4(x0)
    ];
    let _expected_xregs = helper::create_xregs(vec![(16, -5i64 as u64), (17, 3), (18, -5i64 as u64)]);
    let _expected_fregs = helper::create_fregs(vec![]);

    // TODO: fix StoreAMOAccessFault
    //helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn sh_rs2_offset_rs1() {
    let mut _emu = Emulator::new();
    
    let _data = vec![
        0x13, 0x08, 0x00, 0xc0, // addi x16, x0, -1024
        0x93, 0x08, 0x30, 0x00, // addi x17, x0, 3
        0x23, 0x12, 0x00, 0x01, // sh x16, 4(x0)
        0x03, 0x19, 0x40, 0x00, // lh x18, 4(x0)
    ];
    let _expected_xregs = helper::create_xregs(vec![(16, -1024i64 as u64), (17, 3), (18, -1024i64 as u64)]);
    let _expected_fregs = helper::create_fregs(vec![]);

    // TODO: fix StoreAMOAccessFault
    //helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn sw_rs2_offset_rs1() {
    let mut _emu = Emulator::new();
    
    let _data = vec![
        0x13, 0x08, 0x00, 0x80, // addi x16, x0, -2048
        0x93, 0x08, 0x30, 0x00, // addi x17, x0, 3
        0x23, 0x22, 0x00, 0x01, // sw x16, 4(x0)
        0x03, 0x29, 0x40, 0x00, // lw x18, 4(x0)
    ];
    let _expected_xregs = helper::create_xregs(vec![(16, -2048i64 as u64), (17, 3), (18, -2048i64 as u64)]);
    let _expected_fregs = helper::create_fregs(vec![]);

    // TODO: fix StoreAMOAccessFault
    //helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn add_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x93, 0x01, 0x50, 0x00, // addi x3, x0, 5
        0x13, 0x02, 0x60, 0x00, // addi x4, x0, 6
        0x33, 0x81, 0x41, 0x00, // add x2, x3, x4
    ];
    let expected_xregs = helper::create_xregs(vec![(2, 11), (3, 5), (4, 6)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn sub_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x93, 0x01, 0x50, 0x00, // addi x3, x0, 5
        0x13, 0x02, 0x60, 0x00, // addi x4, x0, 6
        0x33, 0x81, 0x41, 0x40, // sub x2, x3, x4
    ];
    let expected_xregs = helper::create_xregs(vec![(2, -1i64 as u64), (3, 5), (4, 6)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn sll_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0x80, 0x00, // addi x16, x0, 8
        0x93, 0x08, 0x20, 0x00, // addi x17, x0, 2
        0x33, 0x19, 0x18, 0x01, // sll x18, x16, x17
    ];
    let expected_xregs = helper::create_xregs(vec![(16, 8), (17, 2), (18, 32)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn slt_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0x80, 0xff, // addi x16, x0, -8
        0x93, 0x08, 0x20, 0x00, // addi x17, x0, 2
        0x33, 0x29, 0x18, 0x01, // slt x18, x16, x17
    ];
    let expected_xregs = helper::create_xregs(vec![(16, -8i64 as u64), (17, 2), (18, 1)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn sltu_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0x80, 0x00, // addi x16, x0, 8
        0x93, 0x08, 0x20, 0x00, // addi x17, x0, 2
        0x33, 0xb9, 0x08, 0x01, // slt x18, x17, x16
    ];
    let expected_xregs = helper::create_xregs(vec![(16, 8), (17, 2), (18, 1)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn xor_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
        0x93, 0x08, 0x60, 0x00, // addi x17, x0, 6
        0x33, 0x49, 0x18, 0x01, // xor x18, x16, x17
    ];
    let expected_xregs = helper::create_xregs(vec![(16, 3), (17, 6), (18, 5)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn srl_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0x00, 0x01, // addi x16, x0, 16
        0x93, 0x08, 0x20, 0x00, // addi x17, x0, 2
        0x33, 0x59, 0x18, 0x01, // srl x18, x16, x17
    ];
    let expected_xregs = helper::create_xregs(vec![(16, 16), (17, 2), (18, 4)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn sra_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0x00, 0xff, // addi x16, x0, -16
        0x93, 0x08, 0x20, 0x00, // addi x17, x0, 2
        0x33, 0x59, 0x18, 0x41, // sra x18, x16, x17
    ];
    let expected_xregs = helper::create_xregs(vec![(16, -16i64 as u64), (17, 2), (18, -4i64 as u64)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn or_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
        0x93, 0x08, 0x50, 0x00, // addi x17, x0, 5
        0x33, 0x69, 0x18, 0x01, // or x18, x16, x17
    ];
    let expected_xregs = helper::create_xregs(vec![(16, 3), (17, 5), (18, 7)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn and_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
        0x93, 0x08, 0x50, 0x00, // addi x17, x0, 5
        0x33, 0x79, 0x18, 0x01, // and x18, x16, x17
    ];
    let expected_xregs = helper::create_xregs(vec![(16, 3), (17, 5), (18, 1)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn lui_rd_imm() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x37, 0x28, 0x00, 0x00, // lui x16, 2
    ];
    let expected_xregs = helper::create_xregs(vec![(16, 8192)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn beq_rs1_rs2_imm() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
        0x93, 0x08, 0x30, 0x00, // addi x17, x0, 3
        0x63, 0x06, 0x18, 0x01, // beq x16, x17, 12
    ];
    let expected_xregs = helper::create_xregs(vec![(16, 3), (17, 3)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);

    assert_eq!(20 + DRAM_BASE, emu.cpu.pc);
}

#[test]
fn bne_rs1_rs2_imm() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
        0x93, 0x08, 0x50, 0x00, // addi x17, x0, 5
        0x63, 0x16, 0x18, 0x01, // bne x16, x17, 12
    ];
    let expected_xregs = helper::create_xregs(vec![(16, 3), (17, 5)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);

    assert_eq!(20 + DRAM_BASE, emu.cpu.pc);
}

#[test]
fn blt_rs1_rs2_imm() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0xd0, 0xff, // addi x16, x0, -3
        0x93, 0x08, 0x50, 0x00, // addi x17, x0, 5
        0x63, 0x46, 0x18, 0x01, // blt x16, x17, 12
    ];
    let expected_xregs = helper::create_xregs(vec![(16, -3i64 as u64), (17, 5)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);

    assert_eq!(20 + DRAM_BASE, emu.cpu.pc);
}

#[test]
fn bge_rs1_rs2_imm() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0xd0, 0xff, // addi x16, x0, -3
        0x93, 0x08, 0xd0, 0xff, // addi x17, x0, -3
        0x63, 0x56, 0x18, 0x01, // bge x16, x17, 12
    ];
    let expected_xregs = helper::create_xregs(vec![(16, -3i64 as u64), (17, -3i64 as u64)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);

    assert_eq!(20 + DRAM_BASE, emu.cpu.pc);
}

#[test]
fn bltu_rs1_rs2_imm() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
        0x93, 0x08, 0x50, 0x00, // addi x17, x0, 5
        0x63, 0x66, 0x18, 0x01, // bltu x16, x17, 12
    ];
    let expected_xregs = helper::create_xregs(vec![(16, 3), (17, 5)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);

    assert_eq!(20 + DRAM_BASE, emu.cpu.pc);
}

#[test]
fn bgeu_rs1_rs2_imm() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0x50, 0x00, // addi x16, x0, 5
        0x93, 0x08, 0x30, 0x00, // addi x17, x0, 3
        0x63, 0x76, 0x18, 0x01, // bgeu x16, x17, 12
    ];
    let expected_xregs = helper::create_xregs(vec![(16, 5), (17, 3)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);

    assert_eq!(20 + DRAM_BASE, emu.cpu.pc);
}

#[test]
fn jalr_rd_imm() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
        0x93, 0x08, 0x50, 0x00, // addi x17, x0, 5
        0x67, 0x09, 0xc0, 0x02, // jalr x18, x0, 44
    ];
    let expected_xregs = helper::create_xregs(vec![(16, 3), (17, 5), (18, 12 + DRAM_BASE)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);

    assert_eq!(44, emu.cpu.pc);
}

#[test]
fn jal_rd_imm() {
    let mut emu = Emulator::new();
    
    let data = vec![
        0x13, 0x08, 0x30, 0x00, // addi x16, x0, 3
        0x93, 0x08, 0x50, 0x00, // addi x17, x0, 5
        0x6f, 0x09, 0xc0, 0x00, // jal x18, 12
    ];
    let expected_xregs = helper::create_xregs(vec![(16, 3), (17, 5), (18, 12 + DRAM_BASE)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);

    assert_eq!(20 + DRAM_BASE, emu.cpu.pc);
}
