#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

use rvemu_core::bus::DRAM_BASE;
use rvemu_core::emulator::Emulator;

wasm_bindgen_test_configure!(run_in_browser);

const DEFAULT_SP: i64 = 1048000 + 0x8000_0000;

#[wasm_bindgen_test]
pub fn illegal_isa() {
    let mut emu = Emulator::new();
    let data = vec![
        0x93, 0x0f, 0x50, 0x00, // addi x31, x0, 5
        0xaa, 0xaa, 0xaa, 0xaa, // Invalid ISA
        0x93, 0x0f, 0x50, 0x00, // addi x31, x0, 5
    ];
    emu.set_dram(data);
    emu.set_pc(DRAM_BASE);

    emu.start();

    let cpu = emu.cpu.lock().expect("failed to get a mutable CPU.");
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
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 5,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
}
