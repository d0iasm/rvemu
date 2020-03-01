#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const DEFAULT_SP: i64 = 1048000;

#[wasm_bindgen_test]
pub fn illegal_isa() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xaa, 0xaa, 0xaa, 0xaa, // Invalid ISA
        ],
    };

    cpu.start(&mut mem);

    assert_eq!(
        2,
        cpu.state
            .read(rvemu::csr::MCAUSE)
            .expect("failed to get mcause")
    );
}
