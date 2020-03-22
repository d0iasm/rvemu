//! The uart module contains the implementation of a universal asynchronous receiver-transmitter
//! (UART) for WebAssembly. The device is 16550a UART, which is used in the QEMU virt machine. See more information
//! in http://byterunner.com/16550.html.

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::MessageEvent;

use crate::bus::{UART_BASE, UART_SIZE};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// The interrupt request of UART.
pub const UART_IRQ: u64 = 10;

/// Receive holding register (for input bytes).
pub const UART_RHR: u64 = UART_BASE + 0;
/// Transmit holding register (for output bytes).
pub const UART_THR: u64 = UART_BASE + 0;
/// Interrupt enable register.
pub const UART_IER: u64 = UART_BASE + 1;
/// FIFO control register.
pub const UART_FCR: u64 = UART_BASE + 2;
/// Interrupt status register.
/// ISR BIT-0:
///     0 = an interrupt is pending and the ISR contents may be used as a pointer to the appropriate
/// interrupt service routine.
///     1 = no interrupt is pending.
pub const UART_ISR: u64 = UART_BASE + 2;
/// Line control register.
pub const UART_LCR: u64 = UART_BASE + 3;
/// Line status register.
/// LSR BIT 0:
///     0 = no data in receive holding register or FIFO.
///     1 = data has been receive and saved in the receive holding register or FIFO.
/// LSR BIT 6:
///     0 = transmitter holding and shift registers are full.
///     1 = transmit holding register is empty. In FIFO mode this bit is set to one whenever the the transmitter FIFO and transmit shift register are empty.
pub const UART_LSR: u64 = UART_BASE + 5;

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

fn get_input() -> u8 {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let buffer = document
        .get_element_by_id("buffer")
        .expect("should have a element with a `buffer` id");

    if buffer.child_element_count() <= 0 {
        return 0;
    }
    let span = buffer
        .first_child()
        .expect("failed to get a first child node");
    let text = span.text_content().expect("failed to get a text content");

    buffer
        .remove_child(&span)
        .expect("faled to remove a first child");

    if text.as_bytes().len() > 0 {
        return text.as_bytes()[0];
    }
    0
}

/// The UART, the size of which is 0x100 (2**8).
pub struct Uart {
    uart: [u8; UART_SIZE as usize],
    clock: u64,
    window: web_sys::Window,
}

impl Uart {
    /// Create a new UART object.
    pub fn new() -> Self {
        let mut uart = [0; UART_SIZE as usize];
        uart[(UART_ISR - UART_BASE) as usize] |= 1;
        uart[(UART_LSR - UART_BASE) as usize] |= 1 << 5;

        let window = web_sys::window().expect("no global `window` exists");
        /*
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            // handle message
            log(&format!("uart get !!!!!!!!! {:?}", e.data()));
            /*
            let response = e
                .data()
                .as_string()
                .expect("Can't convert received data to a string");
                */
            let byte: u8 = e.data().into();
            uart[(UART_LSR - UART_BASE) as usize] |= 1;
        }) as Box<dyn FnMut(MessageEvent)>);
        // set message event handler on WebSocket
        window.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        // forget the callback to keep it alive
        onmessage_callback.forget();
        */

        Self {
            uart,
            clock: 0,
            window,
        }
    }

    /// Return true if the byte buffer in UART is full.
    pub fn is_interrupting(&mut self) -> bool {
        self.clock += 1;
        // Avoid too many interrupting.
        if self.clock >= 100000 {
            self.clock = 0;
            let b = get_input();
            if b == 0 {
                return false;
            }
            self.uart[0] = b;
            log(&format!(
                "uart get input {} {} {}",
                self.uart[0] as char, b as char, b
            ));
            return true;
        }
        false
    }

    /// Read a byte from the receive holding register.
    pub fn read(&mut self, index: u64) -> u8 {
        match index {
            UART_RHR => {
                self.uart[(UART_LSR - UART_BASE) as usize] &= !1;
                self.uart[(index - UART_BASE) as usize]
            }
            _ => self.uart[(index - UART_BASE) as usize],
        }
    }

    /// Write a byte to the transmit holding register.
    pub fn write(&mut self, index: u64, value: u8) {
        match index {
            UART_THR => {
                self.window
                    .post_message(&JsValue::from(value), "*")
                    .expect("failed to post message");
                //write_to_buffer(value);
                //stdout8(value);
            }
            _ => {
                self.uart[(index - UART_BASE) as usize] = value;
            }
        }
    }
}
