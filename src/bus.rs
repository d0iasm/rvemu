//! The bus module contains the system bus which can access the memroy or memory-mapped peripheral
//! devices.

use crate::devices::{clint::Clint, plic::Plic, uart::Uart, virtio_blk::Virtio};
use crate::dram::{Dram, DRAM_SIZE};
use crate::exception::Exception;
use crate::rom::Rom;

// QEMU virt machine:
// https://github.com/qemu/qemu/blob/master/hw/riscv/virt.c#L46-L63

/// The address which the mask ROM starts.
pub const MROM_BASE: u64 = 0x1000;
/// The address which the mask ROM ends.
const MROM_END: u64 = MROM_BASE + 0xf000;

/// The address which the core-local interruptor (CLINT) starts. It contains the timer and generates
/// per-hart software interrupts and timer interrupts.
pub const CLINT_BASE: u64 = 0x200_0000;
/// The address which the core-local interruptor (CLINT) ends.
const CLINT_END: u64 = CLINT_BASE + 0x10000;

/// The address which the platform-level interrupt controller (PLIC) starts. The PLIC connects all
/// external interrupts in the system to all hart contexts in the system, via the external interrupt
/// source in each hart.
pub const PLIC_BASE: u64 = 0xc00_0000;
/// The address which the platform-level interrupt controller (PLIC) ends.
const PLIC_END: u64 = PLIC_BASE + 0x208000;

/// The address which UART starts. QEMU puts UART registers here in physical memory.
pub const UART_BASE: u64 = 0x1000_0000;
/// The size of UART.
pub const UART_SIZE: u64 = 0x100;
/// The address which UART ends.
const UART_END: u64 = UART_BASE + 0x100;

/// The address which virtio starts.
pub const VIRTIO_BASE: u64 = 0x1000_1000;
/// The address which virtio ends.
const VIRTIO_END: u64 = VIRTIO_BASE + 0x1000;

/// The address which DRAM starts.
pub const DRAM_BASE: u64 = 0x8000_0000;
/// The address which DRAM ends.
const DRAM_END: u64 = DRAM_BASE + DRAM_SIZE;

/// The system bus.
pub struct Bus {
    pub clint: Clint,
    pub plic: Plic,
    pub uart: Uart,
    pub virtio: Virtio,
    dram: Dram,
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

    /// Load a `size`-bit data from the device that connects to the system bus.
    pub fn read(&mut self, addr: u64, size: u8) -> Result<u64, Exception> {
        match addr {
            MROM_BASE..=MROM_END => self.rom.read(addr, size),
            CLINT_BASE..=CLINT_END => self.clint.read(addr, size),
            PLIC_BASE..=PLIC_END => self.plic.read(addr, size),
            UART_BASE..=UART_END => self.uart.read(addr, size),
            VIRTIO_BASE..=VIRTIO_END => self.virtio.read(addr, size),
            DRAM_BASE..=DRAM_END => self.dram.read(addr, size),
            _ => Err(Exception::LoadAccessFault),
        }
    }

    /// Store a `size`-bit data to the device that connects to the system bus.
    pub fn write(&mut self, addr: u64, value: u64, size: u8) -> Result<(), Exception> {
        match addr {
            CLINT_BASE..=CLINT_END => self.clint.write(addr, value, size),
            PLIC_BASE..=PLIC_END => self.plic.write(addr, value, size),
            UART_BASE..=UART_END => self.uart.write(addr, value as u8, size),
            VIRTIO_BASE..=VIRTIO_END => self.virtio.write(addr, value as u32, size),
            DRAM_BASE..=DRAM_END => self.dram.write(addr, value, size),
            _ => Err(Exception::StoreAMOAccessFault),
        }
    }
}
