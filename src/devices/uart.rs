//! The uart module contains the implementation of a universal asynchronous receiver-transmitter
//! (UART). The device is 16550a UART, which is used in the QEMU virt machine. See more information
//! in http://byterunner.com/16550.html.

use crate::bus::UART_BASE;
use std::io;
use std::io::prelude::*;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

/// The size of UART.
pub const UART_SIZE: usize = 0x100;

/// Receive holding register (for input bytes).
pub const UART_RHR: usize = UART_BASE + 0;
/// Transmit holding register (for output bytes).
pub const UART_THR: usize = UART_BASE + 0;
/// Interrupt enable register.
pub const UART_IER: usize = UART_BASE + 1;
/// FIFO control register.
pub const UART_FCR: usize = UART_BASE + 2;
/// Interrupt status register.
pub const UART_ISR: usize = UART_BASE + 2;
/// Line control register.
pub const UART_LCR: usize = UART_BASE + 3;
/// Line status register.
/// LSR BIT 1:
/// 0 = no overrun error (normal)
/// 1 = overrun error. A character arived before receive holding register was emptied or if FIFOs are enabled, an overrun error will occur only after the FIFO is full and the next character has been completely received in the shift register. Note that character in the shift register is overwritten, but it is not transferred to the FIFO.
/// LSR BIT 6:
/// 0 = transmitter holding and shift registers are full.
/// 1 = transmit holding register is empty. In FIFO mode this bit is set to one whenever the the transmitter FIFO and transmit shift register are empty.
pub const UART_LSR: usize = UART_BASE + 5;

/// The UART, the size of which is 0x100 (2**8).
pub struct Uart {
    uart: Arc<(Mutex<[u8; UART_SIZE]>, Condvar)>,
}

impl Uart {
    /// Create a new UART object.
    pub fn new() -> Self {
        let uart = Arc::new((Mutex::new([0; UART_SIZE]), Condvar::new()));
        {
            let (uart, _cvar) = &*uart;
            let mut uart = uart.lock().expect("failed to get an UART object");
            uart[UART_LSR - UART_BASE] |= 1 << 5;
        }

        let mut byte = [0; 1];
        let cloned_uart = uart.clone();
        let _uart_thread = thread::spawn(move || loop {
            match io::stdin().read(&mut byte) {
                Ok(_) => {
                    // Wait for the thread to start up.
                    let (uart, cvar) = &*cloned_uart;
                    let mut uart = uart.lock().expect("failed to get an UART object");
                    while (uart[UART_LSR - UART_BASE] & 1) == 1 {
                        uart = cvar.wait(uart).expect("the mutex is poisoned");
                    }
                    uart[0] = byte[0];
                    uart[UART_LSR - UART_BASE] |= 1;
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        });

        Self { uart }
    }

    /// Read a byte from the receive holding register.
    pub fn read(&mut self, index: usize) -> u8 {
        let (uart, cvar) = &*self.uart;
        let mut uart = uart.lock().expect("failed to get an UART object");
        match index {
            UART_RHR => {
                cvar.notify_one();
                uart[UART_LSR - UART_BASE] &= !1;
                uart[index - UART_BASE]
            }
            _ => uart[index - UART_BASE],
        }
    }

    /// Write a byte to the transmit holding register.
    pub fn write(&mut self, index: usize, value: u8) {
        let (uart, _cvar) = &*self.uart;
        let mut uart = uart.lock().expect("failed to get an UART object");
        match index {
            UART_THR => {
                print!("{}", value as char);
                io::stdout().flush().expect("failed to flush stdout");
            }
            _ => {
                uart[index - UART_BASE] = value;
            }
        }
    }
}
