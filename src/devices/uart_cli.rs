//! The uart module contains the implementation of a universal asynchronous receiver-transmitter
//! (UART) for the CLI tool. The device is 16550a UART, which is used in the QEMU virt machine. See more information
//! in http://byterunner.com/16550.html.

use std::io;
use std::io::prelude::*;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use crate::bus::{UART_BASE, UART_SIZE};

/// The interrupt request of UART.
pub const UART_IRQ: usize = 10;

/// Receive holding register (for input bytes).
pub const UART_RHR: usize = UART_BASE + 0;
/// Transmit holding register (for output bytes).
pub const UART_THR: usize = UART_BASE + 0;
/// Interrupt enable register.
pub const UART_IER: usize = UART_BASE + 1;
/// FIFO control register.
pub const UART_FCR: usize = UART_BASE + 2;
/// Interrupt status register.
/// ISR BIT-0:
///     0 = an interrupt is pending and the ISR contents may be used as a pointer to the appropriate
/// interrupt service routine.
///     1 = no interrupt is pending.
pub const UART_ISR: usize = UART_BASE + 2;
/// Line control register.
pub const UART_LCR: usize = UART_BASE + 3;
/// Line status register.
/// LSR BIT 0:
///     0 = no data in receive holding register or FIFO.
///     1 = data has been receive and saved in the receive holding register or FIFO.
/// LSR BIT 6:
///     0 = transmitter holding and shift registers are full.
///     1 = transmit holding register is empty. In FIFO mode this bit is set to one whenever the the transmitter FIFO and transmit shift register are empty.
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
            uart[UART_ISR - UART_BASE] |= 1;
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
                    // An interrupt is pending.
                    uart[UART_ISR - UART_BASE] &= !1;
                    // Data has been receive.
                    uart[UART_LSR - UART_BASE] |= 1;
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        });

        Self { uart }
    }

    /// Return true if an interrupt is pending.
    pub fn is_interrupting(&self) -> bool {
        // TODO: avoid getting a lock too often.
        let (uart, _cvar) = &*self.uart;
        let uart = uart.lock().expect("failed to get an UART object");
        // Check if the bit 0 in ISR is 0.
        (uart[UART_ISR - UART_BASE] & 1) == 0
    }

    /// Set the interrupt pending bit to 1, which means no interrupt is pending.
    pub fn clear_interrupting(&mut self) {
        let (uart, _cvar) = &*self.uart;
        let mut uart = uart.lock().expect("failed to get an UART object");
        // Set the bit 0 in ISR to 1.
        uart[UART_ISR - UART_BASE] |= 1;
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
