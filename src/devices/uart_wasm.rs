//! The uart module contains the implementation of a universal asynchronous receiver-transmitter
//! (UART) for WebAssembly. The device is 16550a UART, which is used in the QEMU virt machine. See more information
//! in http://byterunner.com/16550.html.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::Window;

use crate::bus::{UART_BASE, UART_SIZE};
use crate::cpu::BYTE;
use crate::exception::Exception;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// The interrupt request of UART.
pub const UART_IRQ: u64 = 10;

/// Receive holding register (for input bytes).
const UART_RHR: u64 = UART_BASE + 0;
/// Transmit holding register (for output bytes).
const UART_THR: u64 = UART_BASE + 0;
/// Interrupt enable register.
const _UART_IER: u64 = UART_BASE + 1;
/// FIFO control register.
const _UART_FCR: u64 = UART_BASE + 2;
/// Interrupt status register.
/// ISR BIT-0:
///     0 = an interrupt is pending and the ISR contents may be used as a pointer to the appropriate
/// interrupt service routine.
///     1 = no interrupt is pending.
const UART_ISR: u64 = UART_BASE + 2;
/// Line control register.
const _UART_LCR: u64 = UART_BASE + 3;
/// Line status register.
/// LSR BIT 0:
///     0 = no data in receive holding register or FIFO.
///     1 = data has been receive and saved in the receive holding register or FIFO.
/// LSR BIT 6:
///     0 = transmitter holding and shift registers are full.
///     1 = transmit holding register is empty. In FIFO mode this bit is set to one whenever the the transmitter FIFO and transmit shift register are empty.
const UART_LSR: u64 = UART_BASE + 5;

fn get_input(window: &Window) -> u8 {
    let document = window.document().expect("failed to get a document object");
    let buffer = document
        .get_element_by_id("inputBuffer")
        .expect("failed to get an element by `inputBuffer` id");

    // TODO: take all children
    match buffer.first_child() {
        Some(span) => {
            buffer
                .remove_child(&span)
                .expect("faled to remove a first child");
            let text = span.text_content().expect("failed to get a text content");
            let byte: u8 = text.parse().expect("failed to parse a text to byte");
            byte
        }
        None => 0,
    }
}

/// The UART, the size of which is 0x100 (2**8).
pub struct Uart {
    uart: [u8; UART_SIZE as usize],
    clock: u64,
    not_null: bool,
    window: web_sys::Window,
}

impl Uart {
    /// Create a new UART object.
    pub fn new() -> Self {
        let mut uart = [0; UART_SIZE as usize];
        uart[(UART_ISR - UART_BASE) as usize] |= 1;
        uart[(UART_LSR - UART_BASE) as usize] |= 1 << 5;

        Self {
            uart,
            clock: 0,
            not_null: false,
            window: web_sys::window().expect("failed to get a global window object"),
        }
    }

    /// Return true if the byte buffer in UART is full.
    pub fn is_interrupting(&mut self) -> bool {
        self.clock += 1;
        // Avoid too many interrupting, bus read a byte again if a byte is found in the previous step.
        if self.clock > 500000 || self.not_null {
            self.clock = 0;
            let b = get_input(&self.window);
            if b == 0 {
                self.not_null = false;
                return false;
            }
            self.uart[0] = b;
            self.uart[(UART_LSR - UART_BASE) as usize] |= 1;
            // Found a byte in this step, so it might find a byte again in the next step.
            self.not_null = true;
            return true;
        }
        false
    }

    /// Read a byte from the receive holding register.
    pub fn read(&mut self, index: u64, size: u8) -> Result<u64, Exception> {
        if size != BYTE {
            return Err(Exception::LoadAccessFault);
        }

        match index {
            UART_RHR => {
                self.uart[(UART_LSR - UART_BASE) as usize] &= !1;
                Ok(self.uart[(index - UART_BASE) as usize] as u64)
            }
            _ => Ok(self.uart[(index - UART_BASE) as usize] as u64),
        }
    }

    /// Write a byte to the transmit holding register.
    pub fn write(&mut self, index: u64, value: u8, size: u8) -> Result<(), Exception> {
        if size != BYTE {
            return Err(Exception::StoreAMOAccessFault);
        }

        match index {
            UART_THR => {
                self.window
                    .post_message(&JsValue::from(value), "*")
                    .expect("failed to post message");
            }
            _ => {
                self.uart[(index - UART_BASE) as usize] = value;
            }
        }
        Ok(())
    }
}
