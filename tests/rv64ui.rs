extern crate rvemu;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

use rvemu::{cpu::Cpu, memory::Memory};

const BASE_ADDRESS: usize = 0x80000000;

macro_rules! add_test {
    ($name: ident) => {
        #[test]
        fn $name() -> io::Result<()> {
            let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            root.push("tests/resources");
            root.push(stringify!($name));

            let mut file = File::open(root.as_path())?;
            let mut dram = Vec::new();
            file.read_to_end(&mut dram)?;

            let mut cpu = Cpu::new();
            cpu.pc = BASE_ADDRESS;

            let mut mem = Memory::new();
            for i in 0..dram.len() {
                mem.dram[i] = dram[i];
            }

            cpu.start(&mut mem);

            // Test result is stored at a0 (x10), a function argument and a return value.
            // The riscv-tests set a0 to 0 when all tests pass.
            assert_eq!(0, cpu.xregs.read(10));
            Ok(())
        }
    };
}

add_test!(rv64ui_p_add);
add_test!(rv64ui_p_addi);
add_test!(rv64ui_p_addiw);
add_test!(rv64ui_p_addw);
add_test!(rv64ui_p_and);
add_test!(rv64ui_p_andi);
add_test!(rv64ui_p_auipc);
add_test!(rv64ui_p_beq);
add_test!(rv64ui_p_bge);
add_test!(rv64ui_p_bgeu);
add_test!(rv64ui_p_blt);
add_test!(rv64ui_p_bltu);
add_test!(rv64ui_p_bne);
add_test!(rv64ui_p_fence_i);
add_test!(rv64ui_p_jal);
add_test!(rv64ui_p_jalr);
add_test!(rv64ui_p_lb);
add_test!(rv64ui_p_lbu);
add_test!(rv64ui_p_ld);
add_test!(rv64ui_p_lh);
add_test!(rv64ui_p_lhu);
add_test!(rv64ui_p_lui);
add_test!(rv64ui_p_lw);
add_test!(rv64ui_p_lwu);
add_test!(rv64ui_p_or);
add_test!(rv64ui_p_ori);
add_test!(rv64ui_p_sb);
add_test!(rv64ui_p_sd);
add_test!(rv64ui_p_sh);
add_test!(rv64ui_p_simple);
add_test!(rv64ui_p_sll);
add_test!(rv64ui_p_slli);
add_test!(rv64ui_p_slliw);
add_test!(rv64ui_p_sllw);
add_test!(rv64ui_p_slt);
add_test!(rv64ui_p_slti);
add_test!(rv64ui_p_sltiu);
add_test!(rv64ui_p_sltu);
add_test!(rv64ui_p_sra);
add_test!(rv64ui_p_srai);
add_test!(rv64ui_p_sraiw);
add_test!(rv64ui_p_sraw);
add_test!(rv64ui_p_srl);
add_test!(rv64ui_p_srli);
add_test!(rv64ui_p_srliw);
add_test!(rv64ui_p_srlw);
add_test!(rv64ui_p_sub);
add_test!(rv64ui_p_subw);
add_test!(rv64ui_p_sw);
add_test!(rv64ui_p_xor);
add_test!(rv64ui_p_xori);
