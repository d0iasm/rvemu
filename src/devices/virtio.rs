//! The virtio module contains a virtualization standard for network and disk device drivers.
//! This is the "legacy" virtio interface.
//!
//! The virtio spec:
//! https://docs.oasis-open.org/virtio/virtio/v1.1/virtio-v1.1.pdf

use crate::bus::VIRTIO_BASE;
use crate::cpu::Cpu;

/// The interrupt request of virtio.
pub const VIRTIO_IRQ: usize = 1;

/// Always return 0x74726976.
pub const VIRTIO_MAGIC: usize = VIRTIO_BASE + 0x000;
/// The version. 1 is legacy.
pub const VIRTIO_VERSION: usize = VIRTIO_BASE + 0x004;
/// device type; 1 is net, 2 is disk.
pub const VIRTIO_DEVICE_ID: usize = VIRTIO_BASE + 0x008;
/// Always return 0x554d4551
pub const VIRTIO_VENDOR_ID: usize = VIRTIO_BASE + 0x00c;
/// Device features.
pub const VIRTIO_DEVICE_FEATURES: usize = VIRTIO_BASE + 0x010;
/// Driver features.
pub const VIRTIO_DRIVER_FEATURES: usize = VIRTIO_BASE + 0x020;
/// Page size for PFN, write-only.
pub const VIRTIO_GUEST_PAGE_SIZE: usize = VIRTIO_BASE + 0x028;
/// Select queue, write-only.
pub const VIRTIO_QUEUE_SEL: usize = VIRTIO_BASE + 0x030;
/// Max size of current queue, read-only. In QEMU, `VIRTIO_COUNT = 8`.
pub const VIRTIO_QUEUE_NUM_MAX: usize = VIRTIO_BASE + 0x034;
/// Size of current queue, write-only.
pub const VIRTIO_QUEUE_NUM: usize = VIRTIO_BASE + 0x038;
/// Physical page number for queue, read and write.
pub const VIRTIO_QUEUE_PFN: usize = VIRTIO_BASE + 0x040;
/// Notify the queue number, write-only.
pub const VIRTIO_QUEUE_NOTIFY: usize = VIRTIO_BASE + 0x050;
/// Device status, read and write. Reading from this register returns the current device status flags.
/// Writing non-zero values to this register sets the status flags, indicating the OS/driver
/// progress. Writing zero (0x0) to this register triggers a device reset.
pub const VIRTIO_STATUS: usize = VIRTIO_BASE + 0x070;

/// Paravirtualized drivers for IO virtualization.
pub struct Virtio {
    id: u8,
    driver_features: u32,
    page_size: u32,
    queue_sel: u32,
    queue_num: u32,
    queue_pfn: u32,
    queue_notify: u32,
    status: u32,
    disk: Vec<u8>,
}

impl Virtio {
    /// Create a new virtio object.
    pub fn new() -> Self {
        Self {
            id: 0,
            driver_features: 0,
            page_size: 0,
            queue_sel: 0,
            queue_num: 0,
            queue_pfn: 0,
            queue_notify: 9999, // TODO: what is the correct initial value?
            status: 0,
            disk: Vec::new(),
        }
    }

    /// Return true if an interrupt is pending.
    pub fn is_interrupting(&mut self) -> bool {
        if self.queue_notify != 9999 {
            self.queue_notify = 9999;
            return true;
        }
        false
    }

    /// Set the binary in the virtio disk.
    pub fn set_disk(&mut self, binary: Vec<u8>) {
        self.disk.extend(binary.iter().cloned());
    }

    /// Read 4 bytes from virtio only if the address is valid. Otherwise, return 0.
    pub fn read(&self, addr: usize) -> u32 {
        match addr {
            VIRTIO_MAGIC => 0x74726976,
            VIRTIO_VERSION => 0x1,
            VIRTIO_DEVICE_ID => 0x2,
            VIRTIO_VENDOR_ID => 0x554d4551,
            VIRTIO_DEVICE_FEATURES => 0, // TODO: what should it return?
            VIRTIO_DRIVER_FEATURES => self.driver_features,
            VIRTIO_QUEUE_NUM_MAX => 8,
            VIRTIO_QUEUE_PFN => self.queue_pfn,
            VIRTIO_STATUS => self.status,
            _ => 0,
        }
    }

    /// Write 4 bytes to virtio only if the address is valid. Otherwise, does nothing.
    pub fn write(&mut self, addr: usize, val: u32) {
        match addr {
            VIRTIO_DEVICE_FEATURES => self.driver_features = val,
            VIRTIO_GUEST_PAGE_SIZE => self.page_size = val,
            VIRTIO_QUEUE_SEL => self.queue_sel = val,
            VIRTIO_QUEUE_NUM => self.queue_num = val,
            VIRTIO_QUEUE_PFN => self.queue_pfn = val,
            VIRTIO_QUEUE_NOTIFY => {
                self.queue_notify = val;
                // TODO: call disk_access here.
                //disk_access()
                //interrupt = true;
            }
            VIRTIO_STATUS => self.status = val,
            _ => {}
        }
    }

    fn get_new_id(&mut self) -> u8 {
        self.id = self.id.wrapping_add(1);
        self.id
    }

    fn page_address(&self) -> u64 {
        self.queue_pfn as u64 * self.page_size as u64
    }

    fn read_disk(&self, address: u64) -> u8 {
        self.disk[address as usize]
    }

    fn write_disk(&mut self, address: u64, value: u8) {
        self.disk[address as usize] = value
    }

    /// Access the disk via virtio.
    pub fn disk_access(cpu: &mut Cpu) {
        let avail_address = cpu.bus.virtio.page_address() + 0x40;
        let base_desc_address = cpu.bus.virtio.page_address();
        let base_used_address = cpu.bus.virtio.page_address() + 4096;

        let offset = cpu
            .bus
            .read16(avail_address.wrapping_add(1) as usize)
            .expect("failed to read avail_address");
        let index = cpu
            .bus
            .read16(
                avail_address
                    .wrapping_add(offset as u64 % 8)
                    .wrapping_add(2) as usize,
            )
            .expect("2");
        let desc_size = 16;

        let desc_address0 = (base_desc_address + desc_size * index as u64) as usize;
        let addr0 = cpu.bus.read64(desc_address0).expect("3");
        let next0 = cpu.bus.read16(desc_address0.wrapping_add(14)).expect("6");

        let desc_address1 = (base_desc_address + desc_size * next0 as u64) as usize;
        let addr1 = cpu.bus.read64(desc_address1).expect("7");
        let len1 = cpu.bus.read32(desc_address1.wrapping_add(8)).expect("8");
        let flags1 = cpu.bus.read16(desc_address1.wrapping_add(12)).expect("9");

        let blk_type = cpu.bus.read32(addr0 as usize).unwrap();
        let blk_reserved = cpu.bus.read32(addr0.wrapping_add(4) as usize).unwrap();
        let blk_sector = cpu.bus.read64(addr0.wrapping_add(8) as usize).expect("10");

        match (flags1 & 2) == 0 {
            true => {
                // write to disk
                for i in 0..len1 as u64 {
                    // DMA
                    let data = cpu.bus.read8((addr1 + i) as usize).expect("11");
                    cpu.bus.virtio.write_disk(blk_sector * 512 + i, data);
                }
            }
            false => {
                // read from disk
                for i in 0..len1 as u64 {
                    // DMA
                    let data = cpu.bus.virtio.read_disk(blk_sector * 512 + i);
                    cpu.bus.write8((addr1 + i) as usize, data).expect("12");
                }
            }
        };

        let new_id = cpu.bus.virtio.get_new_id() as u16;
        cpu.bus
            .write16(base_used_address.wrapping_add(2) as usize, new_id % 8)
            .expect("13");
    }
}
