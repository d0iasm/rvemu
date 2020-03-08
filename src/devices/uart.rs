//! The uart module contains the implementation of a universal asynchronous receiver-transmitter
//! (UART). The device is 16550a UART, which is used in the QEMU virt machine. See more information
//! in http://byterunner.com/16550.html.

/// The address which UART starts. QEMU puts UART registers here in physical memory.
pub const UART_BASE: u32 = 0x10000000;

/// Receive holding register (for input bytes).
pub const UART_RHR: u32 = UART_BASE + 0;
/// Transmit holding register (for output bytes).
pub const UART_THR: u32 = UART_BASE + 0;
/// Interrupt enable register.
pub const UART_IER: u32 = UART_BASE + 1;
/// FIFO control register.
pub const UART_FCR: u32 = UART_BASE + 2;
/// Interrupt status register.
pub const UART_ISR: u32 = UART_BASE + 2;
/// Line control register.
pub const UART_LCR: u32 = UART_BASE + 3;
/// Line status register.
pub const UART_LSR: u32 = UART_BASE + 5;

/// The UART, the size of which is 0x100 (2**8).
pub struct Uart {
    uart: [u8; 0x100],
}

impl Uart {
    pub fn new() -> Self {
        Self { uart: [0; 0x100] }
    }

    /// Read a byte from the receive holding register.
    pub fn read(&self) -> u8 {
        self.uart[0]
    }

    /// Write a byte to the transmit holding register.
    pub fn write(&mut self, value: u8) {
        self.uart[0] = value;
    }
}
