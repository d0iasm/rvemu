//! The bus module contains the system bus which can access the memroy or memory-mapped peripheral
//! devices.

use crate::devices::{
    clint::{Clint, CLINT_SIZE},
    uart::{Uart, UART_SIZE},
};
use crate::exception::Exception;
use crate::memory::Memory;

/// Core-Local Interruptor (CLINT). The CLINT block holds memory-mapped control and status
/// registers associated with software and timer interrupts.
pub const CLINT_BASE: usize = 0x200_0000;
/// Platform-Level Interrupt Controller (PLIC). The PLIC connects all external interrupts in the
/// system to all hart contexts in the system, via the external interrupt source in each hart.
pub const PLIC_BASE: usize = 0xc00_0000;
/// The address which UART starts. QEMU puts UART registers here in physical memory.
pub const UART_BASE: usize = 0x1000_0000;
/// The address which DRAM starts.
pub const DRAM_BASE: usize = 0x8000_0000;

/// The system bus.
pub struct Bus {
    clint: Clint,
    uart: Uart,
    pub dram: Memory,
}

impl Bus {
    pub fn new() -> Bus {
        Self {
            clint: Clint::new(),
            uart: Uart::new(),
            dram: Memory::new(),
        }
    }

    /// Return the size of source code in the dram.
    pub fn dram_size(&self) -> usize {
        self.dram.size()
    }

    /// Set the binary data to the memory.
    pub fn set_dram(&mut self, data: Vec<u8>) {
        self.dram.set_dram(data);
    }

    /// Write a byte to the system bus.
    pub fn write8(&mut self, addr: usize, val: u8) -> Result<(), Exception> {
        // TODO: Replace the following code with PMP check (Physical Memory Protection)?
        if CLINT_BASE <= addr && addr < CLINT_BASE + CLINT_SIZE {
            Ok(self.clint.write8(addr, val))
        } else if UART_BASE <= addr && addr < UART_BASE + UART_SIZE {
            Ok(self.uart.write(addr, val))
        } else if DRAM_BASE <= addr {
            Ok(self.dram.write8(addr - DRAM_BASE, val))
        } else {
            // TODO: The type of an exception InstructionAccessFault is correct?
            Err(Exception::InstructionAccessFault)
        }
    }

    /// Write 2 bytes to the system bus.
    pub fn write16(&mut self, addr: usize, val: u16) -> Result<(), Exception> {
        if DRAM_BASE <= addr {
            Ok(self.dram.write16(addr - DRAM_BASE, val))
        } else {
            Err(Exception::InstructionAccessFault)
        }
    }

    /// Write 4 bytes to the system bus.
    pub fn write32(&mut self, addr: usize, val: u32) -> Result<(), Exception> {
        if DRAM_BASE <= addr {
            Ok(self.dram.write32(addr - DRAM_BASE, val))
        } else {
            Err(Exception::InstructionAccessFault)
        }
    }

    /// Write 8 bytes to the system bus.
    pub fn write64(&mut self, addr: usize, val: u64) -> Result<(), Exception> {
        if CLINT_BASE <= addr && addr < CLINT_BASE + CLINT_SIZE {
            Ok(self.clint.write64(addr, val))
        } else if DRAM_BASE <= addr {
            Ok(self.dram.write64(addr - DRAM_BASE, val))
        } else {
            Err(Exception::InstructionAccessFault)
        }
    }

    /// Read a byte from the system bus.
    pub fn read8(&mut self, addr: usize) -> Result<u8, Exception> {
        if CLINT_BASE <= addr && addr < CLINT_BASE + CLINT_SIZE {
            Ok(self.clint.read8(addr))
        } else if UART_BASE <= addr && addr < UART_BASE + UART_SIZE {
            Ok(self.uart.read(addr))
        } else if DRAM_BASE <= addr {
            Ok(self.dram.read8(addr - DRAM_BASE))
        } else {
            Err(Exception::InstructionAccessFault)
        }
    }

    /// Read 2 bytes from the system bus.
    pub fn read16(&self, addr: usize) -> Result<u16, Exception> {
        if DRAM_BASE <= addr {
            Ok(self.dram.read16(addr - DRAM_BASE))
        } else {
            Err(Exception::InstructionAccessFault)
        }
    }

    /// Read 4 bytes from the system bus.
    pub fn read32(&self, addr: usize) -> Result<u32, Exception> {
        if DRAM_BASE <= addr {
            Ok(self.dram.read32(addr - DRAM_BASE))
        } else {
            Err(Exception::InstructionAccessFault)
        }
    }

    /// Read 8 bytes from the system bus.
    pub fn read64(&self, addr: usize) -> Result<u64, Exception> {
        if CLINT_BASE <= addr && addr < CLINT_BASE + CLINT_SIZE {
            Ok(self.clint.read64(addr))
        } else if DRAM_BASE <= addr {
            Ok(self.dram.read64(addr - DRAM_BASE))
        } else {
            Err(Exception::InstructionAccessFault)
        }
    }
}
