use std::io;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

use rvemu_core::cpu::Cpu;
use rvemu_core::devices::uart::*;

/// Input messages from the console in a host computer. Host console -> UART.
pub fn stdin(cpu: Arc<Mutex<Cpu>>) {
    /*
    let mut buffer = Vec::new();
    // TODO: It can detect only after `Enter` key is pressed.
    loop {
        println!("buffer.is_empty() {}", buffer.is_empty());
        if !buffer.is_empty() {
            let mut cpu = cpu.lock().expect("failed to get a mutable CPU");
            cpu.bus
                .write8(UART_THR, buffer.remove(0))
                .expect("failed to write a byte to the UART");
            println!("something comes! {:#?} at cpu pc {}", buffer, cpu.pc);
        } else {
            // let new = io::stdin::bytes().filter_map(|e| e.ok()).collect();
            io::copy(&mut io::stdin(), &mut buffer);
            println!("!!!!!!!!!!!!!!!!!");
            /*
            for byte in io::stdin().bytes() {
                println!("something comes! {:#?}", byte);
                if let Ok(b) = byte {
                    println!("push !!!!!!!!!!!!!!!1 {}, {:#?}", b as char, buffer);
                    buffer.push(b);
                }
                continue
            }
            */
        }
    }

    let mut f = File::open("foo.txt")?;
        let mut buffer = [0; 10];

            // skip to the last 10 bytes of the file
            //     f.seek(SeekFrom::End(-10))?;
            //
            //         // read up to 10 bytes
            //             let n = f.read(&mut buffer)?;
            //
            //                 println!("The bytes: {:?}", &buffer[..n]);
            //                     Ok(())
                               */

    let mut byte = [0; 1];
    loop {
        println!("============== new loop in stdin ================");
        match io::stdin().read(&mut byte) {
            Ok(_) => {
                let mut cpu = cpu.lock().expect("failed to get a mutable CPU");
                // Wait for Transmit Holding Empty to be set in LSR.
                while (cpu
                    .bus
                    .read8(UART_LSR)
                    .expect("failed to get the line statue register (LSR) from UART")
                    & (1 << 5))
                    == 0
                {
                    thread::sleep(time::Duration::from_millis(100));
                    println!("wait {:#?}", cpu.bus.read8(UART_LSR));
                }
                //println!("something comes! {:#?} at cpu pc {}", byte, cpu.pc);
                cpu.bus
                    .write8(UART_THR, byte[0])
                    .expect("failed to write a byte to the UART");
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

/// Output messages to the console in a host computer. UART -> host console.
pub fn stdout(cpu: Arc<Mutex<Cpu>>) {
    loop {
        if let Ok(mut cpu) = cpu.try_lock() {
            // Input data is ready.
            if (cpu.bus.read8(UART_LSR).unwrap() & 0x01) != 0 {
                print!(
                    "THIS IS THE ECHOBACK BYTE: {}\n",
                    cpu.bus
                        .read8(UART_RHR)
                        .expect("faild to get a byte from the UART") as char
                );
            }
        }
    }
}
