//! The bus module contains the system bus which can access the memroy or memory-mapped peripheral
//! devices.

use crate::devices::clint::Clint;
use crate::devices::plic::Plic;
use crate::exception::Exception;
use std::sync::{Arc, Mutex};

use std::ops::RangeInclusive;

/// The address which the core-local interruptor (CLINT) starts. It contains the timer and generates
/// per-hart software interrupts and timer interrupts.
pub const CLINT_BASE: u64 = 0x200_0000;

/// The address which the platform-level interrupt controller (PLIC) starts. The PLIC connects all
/// external interrupts in the system to all hart contexts in the system, via the external interrupt
/// source in each hart.
pub const PLIC_BASE: u64 = 0xc00_0000;

pub trait Device {
    /// Load `size`-bit data from the memory.
    fn read(&mut self, addr: u64, size: u8) -> Result<u64, Exception>;
    /// Store `size`-bit data to the memory.
    fn write(&mut self, addr: u64, value: u64, size: u8) -> Result<(), Exception>;

    fn size(&self) -> u64;

    fn is_interrupting(&mut self) -> bool;

    fn irq(&self) -> Option<u64>;

    fn reset(&mut self);
}

/// The system bus.
pub struct Bus {
    devices: Vec<(RangeInclusive<u64>, Arc<Mutex<dyn Device>>)>,

    pub clint: Clint,
    pub plic: Plic,
    clint_addr_range: RangeInclusive<u64>,
    plic_addr_range: RangeInclusive<u64>,
}

impl Bus {
    /// Create a new bus object.
    pub fn new() -> Bus {
        let clint = Clint::new();
        let plic = Plic::new();
        Self {
            devices: vec![],
            clint_addr_range: (CLINT_BASE..=(CLINT_BASE + clint.size())),
            plic_addr_range: (PLIC_BASE..=(PLIC_BASE + plic.size())),
            clint: clint,
            plic: plic,
        }
    }

    pub fn is_interrupting(&mut self) -> bool {
        let mut saw_interrupt = false;
        for (_, device) in self.devices.iter_mut() {
            let mut device = device.lock().unwrap();
            if device.is_interrupting() {
                if let Some(irq) = device.irq() {
                    self.plic.update_pending(irq);
                    saw_interrupt = true;
                }
            }
        }
        saw_interrupt
    }

    pub fn mount(&mut self, memory_start: u64, device: Arc<Mutex<dyn Device>>) {
        let size = device.lock().unwrap().size();
        let addr_range = memory_start..=(memory_start + size);
        self.devices.push((addr_range, device));
    }

    /// Load a `size`-bit data from the device that connects to the system bus.
    pub fn read(&mut self, addr: u64, size: u8) -> Result<u64, Exception> {
        if self.clint_addr_range.contains(&addr) {
            self.clint.read(addr - self.clint_addr_range.start(), size)
        } else if self.plic_addr_range.contains(&addr) {
            self.plic.read(addr - self.plic_addr_range.start(), size)
        } else {
            for (addr_range, device) in self.devices.iter_mut() {
                if addr_range.contains(&addr) {
                    let mut device = device.lock().unwrap();
                    return device.read(addr - addr_range.start(), size);
                }
            }
            Err(Exception::LoadAccessFault)
        }
    }

    /// Store a `size`-bit data to the device that connects to the system bus.
    pub fn write(&mut self, addr: u64, value: u64, size: u8) -> Result<(), Exception> {
        if self.clint_addr_range.contains(&addr) {
            self.clint
                .write(addr - self.clint_addr_range.start(), value, size)
        } else if self.plic_addr_range.contains(&addr) {
            self.plic
                .write(addr - self.plic_addr_range.start(), value, size)
        } else {
            for (addr_range, device) in self.devices.iter_mut() {
                if addr_range.contains(&addr) {
                    let mut device = device.lock().unwrap();
                    return device.write(addr - addr_range.start(), value, size);
                }
            }
            Err(Exception::StoreAMOAccessFault)
        }
    }
}
