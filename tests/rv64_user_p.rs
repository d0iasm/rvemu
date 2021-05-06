extern crate rvemu;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

use rvemu::{bus::DRAM_BASE, cpu::Mode, emulator::Emulator};

#[macro_export]
macro_rules! add_test {
    ($name: ident) => {
        #[test]
        fn $name() -> io::Result<()> {
            let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            root.push("tests/resources");
            root.push(stringify!($name));

            let mut file = File::open(root.as_path())?;
            let mut data = Vec::new();
            file.read_to_end(&mut data)?;

            let len = data.len() as u64;

            let mut emu = Emulator::new();
            emu.initialize_dram(data);
            emu.initialize_pc(DRAM_BASE);

            emu.test_start(DRAM_BASE, DRAM_BASE + len);

            // Test result is stored at a0 (x10), a function argument and a return value.
            // The riscv-tests set a0 to 0 when all tests pass.
            assert_eq!(0, emu.cpu.xregs.read(10));

            // All tests start the user mode and finish with the instruction `ecall`, independently
            // of it succeeds or fails.
            assert_eq!(Mode::Machine, emu.cpu.mode);
            Ok(())
        }
    };
}

// rv64ui-p-*
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

/*
// rv64ui-v-*
add_test!(rv64ui_v_add);
add_test!(rv64ui_v_addi);
add_test!(rv64ui_v_addiw);
add_test!(rv64ui_v_addw);
add_test!(rv64ui_v_and);
add_test!(rv64ui_v_andi);
add_test!(rv64ui_v_auipc);
add_test!(rv64ui_v_beq);
add_test!(rv64ui_v_bge);
add_test!(rv64ui_v_bgeu);
add_test!(rv64ui_v_blt);
add_test!(rv64ui_v_bltu);
add_test!(rv64ui_v_bne);
add_test!(rv64ui_v_fence_i);
add_test!(rv64ui_v_jal);
add_test!(rv64ui_v_jalr);
add_test!(rv64ui_v_lb);
add_test!(rv64ui_v_lbu);
add_test!(rv64ui_v_ld);
add_test!(rv64ui_v_lh);
add_test!(rv64ui_v_lhu);
add_test!(rv64ui_v_lui);
add_test!(rv64ui_v_lw);
add_test!(rv64ui_v_lwu);
add_test!(rv64ui_v_or);
add_test!(rv64ui_v_ori);
add_test!(rv64ui_v_sb);
add_test!(rv64ui_v_sd);
add_test!(rv64ui_v_sh);
add_test!(rv64ui_v_simple);
add_test!(rv64ui_v_sll);
add_test!(rv64ui_v_slli);
add_test!(rv64ui_v_slliw);
add_test!(rv64ui_v_sllw);
add_test!(rv64ui_v_slt);
add_test!(rv64ui_v_slti);
add_test!(rv64ui_v_sltiu);
add_test!(rv64ui_v_sltu);
add_test!(rv64ui_v_sra);
add_test!(rv64ui_v_srai);
add_test!(rv64ui_v_sraiw);
add_test!(rv64ui_v_sraw);
add_test!(rv64ui_v_srl);
add_test!(rv64ui_v_srli);
add_test!(rv64ui_v_srliw);
add_test!(rv64ui_v_srlw);
add_test!(rv64ui_v_sub);
add_test!(rv64ui_v_subw);
add_test!(rv64ui_v_sw);
add_test!(rv64ui_v_xor);
add_test!(rv64ui_v_xori);
*/

// rv64ua-p-*
add_test!(rv64ua_p_amoadd_d);
add_test!(rv64ua_p_amoadd_w);
add_test!(rv64ua_p_amoand_d);
add_test!(rv64ua_p_amoand_w);
add_test!(rv64ua_p_amomax_d);
add_test!(rv64ua_p_amomax_w);
add_test!(rv64ua_p_amomaxu_d);
add_test!(rv64ua_p_amomaxu_w);
add_test!(rv64ua_p_amomin_d);
add_test!(rv64ua_p_amomin_w);
add_test!(rv64ua_p_amominu_d);
add_test!(rv64ua_p_amominu_w);
add_test!(rv64ua_p_amoor_d);
add_test!(rv64ua_p_amoor_w);
add_test!(rv64ua_p_amoswap_d);
add_test!(rv64ua_p_amoswap_w);
add_test!(rv64ua_p_amoxor_d);
add_test!(rv64ua_p_amoxor_w);
//TODO fix
//add_test!(rv64ua_p_lrsc);

// rv64ud-p-*
/*
add_test!(rv64ud_p_fadd);
add_test!(rv64ud_p_fclass);
add_test!(rv64ud_p_fcmp);
add_test!(rv64ud_p_fcvt);
add_test!(rv64ud_p_fcvt_w);
add_test!(rv64ud_p_fdiv);
add_test!(rv64ud_p_fmadd);
add_test!(rv64ud_p_fmin);
add_test!(rv64ud_p_ldst);
add_test!(rv64ud_p_move);
add_test!(rv64ud_p_recoding);
add_test!(rv64ud_p_structural);
*/

// rv64uf-p-*
/*
add_test!(rv64uf_p_fadd);
add_test!(rv64uf_p_fclass);
add_test!(rv64uf_p_fcmp);
add_test!(rv64uf_p_fcvt);
add_test!(rv64uf_p_fcvt_w);
add_test!(rv64uf_p_fdiv);
add_test!(rv64uf_p_fmadd);
add_test!(rv64uf_p_fmin);
add_test!(rv64uf_p_ldst);
add_test!(rv64uf_p_move);
add_test!(rv64uf_p_recoding);
*/

// rv64um-p-*
add_test!(rv64um_p_div);
add_test!(rv64um_p_divu);
add_test!(rv64um_p_divuw);
add_test!(rv64um_p_divw);
add_test!(rv64um_p_mul);
add_test!(rv64um_p_mulh);
add_test!(rv64um_p_mulhsu);
add_test!(rv64um_p_mulhu);
add_test!(rv64um_p_mulw);
add_test!(rv64um_p_rem);
add_test!(rv64um_p_remu);
add_test!(rv64um_p_remuw);
add_test!(rv64um_p_remw);

// rv64uc-p-*
add_test!(rv64uc_p_rvc);

// rv64mi-p-*
add_test!(rv64mi_p_access);
//add_test!(rv64mi_p_breakpoint);
//add_test!(rv64mi_p_csr);
//add_test!(rv64mi_p_illegal);
add_test!(rv64mi_p_ma_addr);
//add_test!(rv64mi_p_ma_fetch);
add_test!(rv64mi_p_mcsr);
add_test!(rv64mi_p_sbreak);
add_test!(rv64mi_p_scall);

// rv64si-p-*
//add_test!(rv64si_p_csr);
//add_test!(rv64si_p_dirty);
//add_test!(rv64si_p_icache_alias);
add_test!(rv64si_p_ma_fetch);
add_test!(rv64si_p_sbreak);
add_test!(rv64si_p_scall);
//add_test!(rv64si_p_wfi);

//add_test!(rv64ua_v_lrsc);
