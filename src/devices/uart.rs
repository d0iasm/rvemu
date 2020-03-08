//! The uart module contains the implementation of a universal asynchronous receiver-transmitter
//! (UART). The device is 16550a UART, which is used in the QEMU virt machine. See more information
//! in http://byterunner.com/16550.html.

/// The address which UART starts. QEMU puts UART registers here in physical memory.
pub const UART_BASE:u32 =  0x10000000;

/// Receive holding register (for input bytes).
pub const UART_RHR = UART_BASE + 0;
/// Transmit holding register (for output bytes).
pub const UART_THR = UART_BASE + 0;
/// Interrupt enable register.
pub const UART_IER = UART_BASE + 1;
/// FIFO control register.
pub const UART_FCR = UART_BASE + 2;
/// Interrupt status register.
pub const UART_ISR = UART_BASE + 2;
/// Line control register.
pub const UART_LCR = UART_BASE + 3;
/// Line status register.
pub const UART_LSR = UART_BASE + 5;
