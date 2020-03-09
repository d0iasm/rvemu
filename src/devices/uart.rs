//! The uart module contains the implementation of a universal asynchronous receiver-transmitter
//! (UART). The device is 16550a UART, which is used in the QEMU virt machine. See more information
//! in http://byterunner.com/16550.html.

use crate::bus::UART_BASE;

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
    uart: [u8; 0x100],
}

impl Uart {
    pub fn new() -> Self {
        let mut uart = [0; 0x100];
        uart[UART_LSR - UART_BASE] |= 1 << 5;
        Self { uart }
    }

    /// Read a byte from the receive holding register.
    pub fn read(&mut self, index: usize) -> u8 {
        match index {
            UART_RHR => {
                self.uart[UART_LSR - UART_BASE] &= !1;
                self.uart[UART_LSR - UART_BASE] |= 1 << 5;
                self.uart[index - UART_BASE]
            }
            _ => self.uart[index - UART_BASE],
        }
    }

    /// Write a byte to the transmit holding register.
    pub fn write(&mut self, index: usize, value: u8) {
        match index {
            UART_THR => {
                self.uart[UART_LSR - UART_BASE] |= 1;
                self.uart[UART_LSR - UART_BASE] &= !(1 << 5);
                self.uart[index - UART_BASE] = value;
                //print!("!!!!!!!!!!!!!!!!! {}\n", value as char);
            }
            _ => {
                self.uart[index - UART_BASE] = value;
            }
        }
    }

    pub fn size(&self) -> usize {
        self.uart.len()
    }
}
