extern crate rvemu;

/*
use rvemu::{cpu::Cpu, memory::Memory};

#[test]
fn debug() {
    let mut cpu = Cpu::new();
    let mut mem = Memory {
        dram: vec![
            0x67, 0x00, 0xc3, 0xff, // jr      -4(t1)
        ],
    };

    cpu.start(&mut mem);

    for i in 0..32 {
        dbg!("{:#?}", cpu.xregs.read(i));
    }
    dbg!("pc {}",cpu.pc);
}
*/
