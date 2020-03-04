extern crate rvemu;

use rvemu::{cpu::Cpu, memory::Memory};

#[test]
fn debug() {
    /*
    let mut cpu = Cpu::new();
    let mut mem = Memory {
        dram: vec![
            0x73, 0x25, 0x40, 0xf1, // csrr    a0,mhartid
            0x93, 0x82, 0x02, 0x01, // addi    t0,t0,16
            0x73, 0x90, 0x52, 0x30, // csrw    mtvec,t0
        ],
    };

    cpu.start(&mut mem);

    for i in 0..32 {
        dbg!("{:#?}", cpu.xregs.read(i));
    }
    */
}
