use std::io;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

use rvemu_core::cpu::Cpu;

/// Input a message from the console in a host computer.
pub fn stdin(cpu: Arc<Mutex<Cpu>>) {
    // TODO: It can detect only after `Enter` key is pressed.
    loop {
        for byte in io::stdin().bytes() {
            match byte {
                Ok(b) => {
                    println!("get stdin!!!!");
                    let mut cpu = cpu.lock().expect("failed to get a mutable CPU.");
                    println!(
                        "something comes! {:#?} {} at cpu pc {}",
                        b, b as char, cpu.pc
                    );
                }
                Err(e) => {
                    println!("{:#?}", e);
                }
            }
        }
    }
}

pub fn stdout() {}
