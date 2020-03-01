#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const DEFAULT_SP: i64 = 1048000;

#[wasm_bindgen_test]
pub fn illegal_isa() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x93, 0x0f, 0x50, 0x00, // addi x31, x0, 5
            0xaa, 0xaa, 0xaa, 0xaa, // Invalid ISA
            0x93, 0x0f, 0x50, 0x00, // addi x31, x0, 5
        ],
    };

    cpu.start(&mut mem);

    assert_eq!(
        2,
        cpu.state
            .read(rvemu::csr::MCAUSE)
            .expect("failed to get mcause")
    );

    assert_eq!(
        4,
        cpu.state
            .read(rvemu::csr::MEPC)
            .expect("failed to get mepc")
    );

    let expected = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 5,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}
