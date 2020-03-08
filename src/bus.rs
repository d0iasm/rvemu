//! The bus module contains the system bus which can access the memroy or memory-mapped peripheral
//! devices.

use crate::devices::uart::Uart;
use crate::exception::Exception;
use crate::memory::Memory;

/// The address which UART starts. QEMU puts UART registers here in physical memory.
pub const UART_BASE: usize = 0x1000_0000;
/// The address which DRAM starts.
pub const DRAM_BASE: usize = 0x8000_0000;

/// The system bus.
pub struct Bus {
    uart: Uart,
    pub dram: Memory,
}

impl Bus {
    pub fn new() -> Bus {
        Self {
            uart: Uart::new(),
            dram: Memory::new(),
        }
    }

    /// Return the size of source code in the dram.
    pub fn dram_size(&self) -> usize {
        self.dram.size()
    }

    /// Set the binary to the memory.
    pub fn set_dram(&mut self, binary: Vec<u8>) {
        self.dram.set_dram(binary);
    }

    /// Write a byte to the system bus.
    pub fn write8(&mut self, index: usize, val: u8) -> Result<(), Exception> {
        // TODO: Replace the following code with PMP check (Physical Memory Protection)?
        if UART_BASE <= index && index < UART_BASE + 0x100 {
            Ok(self.uart.write(val))
        } else if DRAM_BASE <= index {
            let physical = index - DRAM_BASE;
            Ok(self.dram.write8(physical, val))
        } else {
            // TODO: The type of an exception InstructionAccessFault is correct?
            Err(Exception::InstructionAccessFault)
        }
    }

    /// Write 2 bytes to the system bus.
    pub fn write16(&mut self, index: usize, val: u16) -> Result<(), Exception> {
        if DRAM_BASE <= index {
            let physical = index - DRAM_BASE;
            Ok(self.dram.write16(physical, val))
        } else {
            Err(Exception::InstructionAccessFault)
        }
    }

    /// Write 4 bytes to the system bus.
    pub fn write32(&mut self, index: usize, val: u32) -> Result<(), Exception> {
        if DRAM_BASE <= index {
            let physical = index - DRAM_BASE;
            Ok(self.dram.write32(physical, val))
        } else {
            Err(Exception::InstructionAccessFault)
        }
    }

    /// Write 8 bytes to the system bus.
    pub fn write64(&mut self, index: usize, val: u64) -> Result<(), Exception> {
        if DRAM_BASE <= index {
            let physical = index - DRAM_BASE;
            Ok(self.dram.write64(physical, val))
        } else {
            Err(Exception::InstructionAccessFault)
        }
    }

    /// Read a byte from the system bus.
    pub fn read8(&self, index: usize) -> Result<u8, Exception> {
        if UART_BASE <= index && index < UART_BASE + 0x100 {
            Ok(self.uart.read())
        } else if DRAM_BASE <= index {
            let physical = index - DRAM_BASE;
            Ok(self.dram.read8(physical))
        } else {
            Err(Exception::InstructionAccessFault)
        }
    }

    /// Read 2 bytes from the system bus.
    pub fn read16(&self, index: usize) -> Result<u16, Exception> {
        if DRAM_BASE <= index {
            let physical = index - DRAM_BASE;
            Ok(self.dram.read16(physical))
        } else {
            Err(Exception::InstructionAccessFault)
        }
    }

    /// Read 4 bytes from the system bus.
    pub fn read32(&self, index: usize) -> Result<u32, Exception> {
        if DRAM_BASE <= index {
            let physical = index - DRAM_BASE;
            Ok(self.dram.read32(physical))
        } else {
            Err(Exception::InstructionAccessFault)
        }
    }

    /// Read 8 bytes from the system bus.
    pub fn read64(&self, index: usize) -> Result<u64, Exception> {
        if DRAM_BASE <= index {
            let physical = index - DRAM_BASE;
            Ok(self.dram.read64(physical))
        } else {
            Err(Exception::InstructionAccessFault)
        }
    }
}
