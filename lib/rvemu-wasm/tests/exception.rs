#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

use rvemu_core::bus::DRAM_BASE;

wasm_bindgen_test_configure!(run_in_browser);

const DEFAULT_SP: i64 = 1048000;

#[wasm_bindgen_test]
pub fn illegal_isa() {
    let mut cpu = rvemu_core::cpu::Cpu::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let dram = vec![
        0x93, 0x0f, 0x50, 0x00, // addi x31, x0, 5
        0xaa, 0xaa, 0xaa, 0xaa, // Invalid ISA
        0x93, 0x0f, 0x50, 0x00, // addi x31, x0, 5
    ];
    bus.dram.dram.splice(..dram.len(), dram.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus, || ());

    assert_eq!(8 + DRAM_BASE, cpu.pc);

    assert_eq!(
        2,
        cpu.state
            .read(rvemu_core::csr::MCAUSE)
            .expect("failed to get mcause")
    );

    assert_eq!(
        (4 + DRAM_BASE) as i64,
        cpu.state
            .read(rvemu_core::csr::MEPC)
            .expect("failed to get mepc")
    );

    let expected = [
        0, 0, DEFAULT_SP + DRAM_BASE as i64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 5,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}
