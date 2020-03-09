use std::io;
use std::io::prelude::*;

/// Input a message from the console in a host computer.
pub fn stdin() {
    // TODO: It can detect only after `Enter` key is pressed.
    loop {
        for byte in io::stdin().bytes() {
            match byte {
                Ok(b) => {
                    println!("something comes! {:#?} {}", b, b as char);
                }
                Err(e) => {
                    println!("{:#?}", e);
                }
            }
        }
    }
}

pub fn stdout() {}
