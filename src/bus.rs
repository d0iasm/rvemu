//! The bus module contains the system bus which can access the memroy or memory-mapped peripheral
//! devices.

use crate::devices::uart::Uart;
use crate::exception::Exception;
use crate::memory::Memory;

/// The system bus.
pub struct Bus {
    uart: Uart,
    dram: Memory,
}

impl Bus {
    pub fn new() -> Bus {
        Self {
            uart: Uart::new(),
            dram: Memory::new(),
        }
    }

    /// Write a byte to the system bus.
    pub fn write8(&mut self, index: usize, val: u8) -> Result<(), Exception> {
        // TODO: Replace the following code with PMP check (Physical Memory Protection)?
        if 0x10000000 <= index && index < 0x10000000 + 0x100 {
            Ok(self.uart.write(val))
        } else if 0x80000000 <= index {
            Ok(self.dram.write8(index, val))
        } else {
            // TODO: The type of an exception InstructionAccessFault is correct?
            Err(Exception::InstructionAccessFault)
        }
    }

    /// Write 2 bytes to the system bus.
    pub fn write16(&mut self, index: usize, val: u16) -> Result<(), Exception> {
        if 0x80000000 <= index {
            Ok(self.dram.write16(index, val))
        } else {
            Err(Exception::InstructionAccessFault)
        }
    }

    /// Write 4 bytes to the system bus.
    pub fn write32(&mut self, index: usize, val: u32) -> Result<(), Exception> {
        if 0x80000000 <= index {
            Ok(self.dram.write32(index, val))
        } else {
            Err(Exception::InstructionAccessFault)
        }
    }

    /// Write 8 bytes to the system bus.
    pub fn write64(&mut self, index: usize, val: u64) -> Result<(), Exception> {
        if 0x80000000 <= index {
            Ok(self.dram.write64(index, val))
        } else {
            Err(Exception::InstructionAccessFault)
        }
    }

    /// Read a byte from the system bus.
    pub fn read8(&mut self, index: usize) -> Result<u8, Exception> {
        if 0x10000000 <= index && index < 0x10000000 + 0x100 {
            Ok(self.uart.read())
        } else if 0x80000000 <= index {
            Ok(self.dram.read8(index))
        } else {
            Err(Exception::InstructionAccessFault)
        }
    }

    /// Read 2 bytes from the system bus.
    pub fn read16(&mut self, index: usize) -> Result<u16, Exception> {
        if 0x80000000 <= index {
            Ok(self.dram.read16(index))
        } else {
            Err(Exception::InstructionAccessFault)
        }
    }

    /// Read 4 bytes from the system bus.
    pub fn read32(&mut self, index: usize) -> Result<u32, Exception> {
        if 0x80000000 <= index {
            Ok(self.dram.read32(index))
        } else {
            Err(Exception::InstructionAccessFault)
        }
    }

    /// Read 8 bytes from the system bus.
    pub fn read64(&mut self, index: usize) -> Result<u64, Exception> {
        if 0x80000000 <= index {
            Ok(self.dram.read64(index))
        } else {
            Err(Exception::InstructionAccessFault)
        }
    }
}
