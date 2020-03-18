//! The uart module contains the implementation of a universal asynchronous receiver-transmitter
//! (UART) for WebAssembly. The device is 16550a UART, which is used in the QEMU virt machine. See more information
//! in http://byterunner.com/16550.html.

use wasm_bindgen::prelude::*;

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

/// Output a message to the emulator console.
pub fn stdout8(byte: u8) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let buffer = document
        .get_element_by_id("buffer8")
        .expect("should have a element with a `buffer8` id");

    let message = format!("{}", byte as char);
    let span = document
        .create_element("span")
        .expect("span element should be created successfully");
    span.set_inner_html(&message);
    let result = buffer.append_child(&span);
    if result.is_err() {
        panic!("can't append a span node to a buffer node")
    }
}

#[wasm_bindgen]
/// The UART, the size of which is 0x100 (2**8).
pub struct Uart {
    uart: [u8; UART_SIZE],
}

#[wasm_bindgen]
impl Uart {
    /// Create a new UART object.
    pub fn new() -> Self {
        let mut uart = [0; UART_SIZE];
        uart[UART_ISR - UART_BASE] |= 1;
        uart[UART_LSR - UART_BASE] |= 1 << 5;
        Self { uart }
    }

    /// Return true if the byte buffer in UART is full.
    pub fn is_interrupting(&self) -> bool {
        (self.uart[UART_ISR - UART_BASE] & 1) == 0
    }

    /// Set the interrupt pending bit to 1, which means no interrupt is pending.
    pub fn clear_interrupting(&mut self) {
        self.uart[UART_ISR - UART_BASE] |= 1;
    }

    /// Read a byte from the receive holding register.
    pub fn read(&mut self, index: usize) -> u8 {
        match index {
            UART_RHR => {
                self.uart[UART_LSR - UART_BASE] &= !1;
                self.uart[index - UART_BASE]
            }
            _ => self.uart[index - UART_BASE],
        }
    }

    /// Write a byte to the transmit holding register.
    pub fn write(&mut self, index: usize, value: u8) {
        match index {
            UART_THR => {
                stdout8(value);
            }
            _ => {
                self.uart[index - UART_BASE] = value;
            }
        }
    }
}
