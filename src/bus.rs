//! The bus module contains the system bus which can access the memroy or memory-mapped peripheral
//! devices.

use crate::devices::{clint::Clint, plic::Plic, uart::Uart, virtio::Virtio};
use crate::dram::{Dram, DRAM_SIZE};
use crate::exception::Exception;
use crate::rom::Rom;

// QEMU virt machine:
// https://github.com/qemu/qemu/blob/master/hw/riscv/virt.c#L46-L63

/// The address which the debug information includes.
pub const DEBUG_BASE: u64 = 0x0;
/// The size of debug information.
pub const DEBUG_SIZE: u64 = 0x100;
pub const DEBUG_END: u64 = DEBUG_BASE + 0x100;

/// The address which the mask ROM starts.
pub const MROM_BASE: u64 = 0x1000;
/// The size of the mask ROM.
pub const MROM_SIZE: u64 = 0xf000;
pub const MROM_END: u64 = MROM_BASE + 0xf000;

/// The address which the core-local interruptor (CLINT) starts. It contains the timer and generates
/// per-hart software interrupts and timer interrupts.
pub const CLINT_BASE: u64 = 0x200_0000;
/// The size of CLINT.
pub const CLINT_SIZE: u64 = 0x10000;
pub const CLINT_END: u64 = CLINT_BASE + 0x10000;

/// The address which the platform-level interrupt controller (PLIC) starts. The PLIC connects all
/// external interrupts in the system to all hart contexts in the system, via the external interrupt
/// source in each hart.
pub const PLIC_BASE: u64 = 0xc00_0000;
/// The size of PLIC.
pub const PLIC_SIZE: u64 = 0x208000;
pub const PLIC_END: u64 = PLIC_BASE + 0x208000;

/// The address which UART starts. QEMU puts UART registers here in physical memory.
pub const UART_BASE: u64 = 0x1000_0000;
/// The size of UART.
pub const UART_SIZE: u64 = 0x100;
pub const UART_END: u64 = UART_BASE + 0x100;

/// The address which virtio starts.
pub const VIRTIO_BASE: u64 = 0x1000_1000;
/// The size of virtio.
pub const VIRTIO_SIZE: u64 = 0x1000;
pub const VIRTIO_END: u64 = VIRTIO_BASE + 0x1000;

/// The address which DRAM starts.
pub const DRAM_BASE: u64 = 0x8000_0000;
pub const DRAM_END: u64 = DRAM_BASE + DRAM_SIZE;

/// The system bus.
pub struct Bus {
    pub clint: Clint,
    pub plic: Plic,
    pub uart: Uart,
    pub virtio: Virtio,
    pub dram: Dram,
    pub rom: Rom,
}

impl Bus {
    /// Create a new bus object.
    pub fn new() -> Bus {
        Self {
            clint: Clint::new(),
            plic: Plic::new(),
            uart: Uart::new(),
            virtio: Virtio::new(),
            dram: Dram::new(),
            rom: Rom::new(),
        }
    }

    /// Set the binary data to the memory.
    pub fn initialize_dram(&mut self, data: Vec<u8>) {
        self.dram.initialize(data);
    }

    /// Set the binary data to the virtIO disk.
    pub fn initialize_disk(&mut self, data: Vec<u8>) {
        self.virtio.initialize(data);
    }

    /// Read a byte from the system bus.
    pub fn read8(&mut self, addr: u64) -> Result<u64, Exception> {
        match addr {
            MROM_BASE..=MROM_END => Ok(self.rom.read8(addr)),
            CLINT_BASE..=CLINT_END => self.clint.read(addr, 8),
            UART_BASE..=UART_END => Ok(self.uart.read(addr) as u64),
            DRAM_BASE..=DRAM_END => Ok(self.dram.read8(addr)),
            _ => Err(Exception::InstructionAccessFault),
        }
    }

    /// Read 2 bytes from the system bus.
    pub fn read16(&self, addr: u64) -> Result<u64, Exception> {
        if MROM_BASE <= addr && addr < MROM_BASE + MROM_SIZE {
            return Ok(self.rom.read16(addr));
        }
        if CLINT_BASE <= addr && addr < CLINT_BASE + CLINT_SIZE {
            return self.clint.read(addr, 16);
        }
        if DRAM_BASE <= addr && addr < DRAM_BASE + DRAM_SIZE {
            return Ok(self.dram.read16(addr));
        }
        Err(Exception::InstructionAccessFault)
    }

    /// Read 4 bytes from the system bus.
    pub fn read32(&self, addr: u64) -> Result<u64, Exception> {
        if DEBUG_BASE <= addr && addr < DEBUG_BASE + DEBUG_SIZE {
            // Nothing for now.
            return Ok(0);
        }
        if MROM_BASE <= addr && addr < MROM_BASE + MROM_SIZE {
            return Ok(self.rom.read32(addr));
        }
        if CLINT_BASE <= addr && addr < CLINT_BASE + CLINT_SIZE {
            return self.clint.read(addr, 32);
        }
        if PLIC_BASE <= addr && addr < PLIC_BASE + PLIC_SIZE {
            return self.plic.read32(addr);
        }
        if VIRTIO_BASE <= addr && addr < VIRTIO_BASE + VIRTIO_SIZE {
            return Ok(self.virtio.read(addr) as u64);
        }
        if DRAM_BASE <= addr && addr < DRAM_BASE + DRAM_SIZE {
            return Ok(self.dram.read32(addr));
        }
        Err(Exception::InstructionAccessFault)
    }

    /// Read 8 bytes from the system bus.
    pub fn read64(&self, addr: u64) -> Result<u64, Exception> {
        if MROM_BASE <= addr && addr < MROM_BASE + MROM_SIZE {
            return Ok(self.rom.read64(addr));
        }
        if CLINT_BASE <= addr && addr < CLINT_BASE + CLINT_SIZE {
            return self.clint.read(addr, 64);
        }
        if PLIC_BASE <= addr && addr < PLIC_BASE + PLIC_SIZE {
            // TODO: make read64 for plic.
            return self.plic.read32(addr);
        }
        if DRAM_BASE <= addr && addr < DRAM_BASE + DRAM_SIZE {
            return Ok(self.dram.read64(addr));
        }
        Err(Exception::InstructionAccessFault)
    }

    /// Write a byte to the system bus.
    pub fn write8(&mut self, addr: u64, val: u64) -> Result<(), Exception> {
        if CLINT_BASE <= addr && addr < CLINT_BASE + CLINT_SIZE {
            return self.clint.write(addr, val, 8);
        }
        // TODO: Replace the following code with PMP check (Physical Memory Protection)?
        if UART_BASE <= addr && addr < UART_BASE + UART_SIZE {
            return Ok(self.uart.write(addr, val as u8));
        }
        if DRAM_BASE <= addr && addr < DRAM_BASE + DRAM_SIZE {
            return Ok(self.dram.write8(addr, val));
        }
        // TODO: The type of an exception InstructionAccessFault is correct?
        Err(Exception::InstructionAccessFault)
    }

    /// Write 2 bytes to the system bus.
    pub fn write16(&mut self, addr: u64, val: u64) -> Result<(), Exception> {
        if CLINT_BASE <= addr && addr < CLINT_BASE + CLINT_SIZE {
            return self.clint.write(addr, val, 16);
        }
        if DRAM_BASE <= addr && addr < DRAM_BASE + DRAM_SIZE {
            return Ok(self.dram.write16(addr, val));
        }
        Err(Exception::InstructionAccessFault)
    }

    /// Write 4 bytes to the system bus.
    pub fn write32(&mut self, addr: u64, val: u64) -> Result<(), Exception> {
        if CLINT_BASE <= addr && addr < CLINT_BASE + CLINT_SIZE {
            return self.clint.write(addr, val, 32);
        }
        if PLIC_BASE <= addr && addr < PLIC_BASE + PLIC_SIZE {
            return self.plic.write32(addr, val as u32);
        }
        if VIRTIO_BASE <= addr && addr < VIRTIO_BASE + VIRTIO_SIZE {
            return Ok(self.virtio.write(addr, val as u32));
        }
        if DRAM_BASE <= addr && addr < DRAM_BASE + DRAM_SIZE {
            return Ok(self.dram.write32(addr, val));
        }
        Err(Exception::InstructionAccessFault)
    }

    /// Write 8 bytes to the system bus.
    pub fn write64(&mut self, addr: u64, val: u64) -> Result<(), Exception> {
        if CLINT_BASE <= addr && addr < CLINT_BASE + CLINT_SIZE {
            return self.clint.write(addr, val, 64);
        }
        if PLIC_BASE <= addr && addr < PLIC_BASE + PLIC_SIZE {
            // TODO: make write64 for plic.
            return self.plic.write32(addr, val as u32);
        }
        if DRAM_BASE <= addr && addr < DRAM_BASE + DRAM_SIZE {
            return Ok(self.dram.write64(addr, val));
        }
        Err(Exception::InstructionAccessFault)
    }
}
