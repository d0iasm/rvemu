#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn debug() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x93, 0x01, 0x50, 0x00, // addi x3, x0, 5
            0x13, 0x02, 0x60, 0x00, // addi x4, x0, 6
            0x33, 0x81, 0x41, 0x00, // add x2, x3, x4
        ],
    };

    cpu.start(&mut mem);

    let elf = rvemu::elf::Elf64Ehdr::new(&mem.dram);
}
