//! The virtio module contains a virtualization standard for network and disk device drivers.
//! This is the "legacy" virtio interface.
//!
//! The virtio spec:
//! https://docs.oasis-open.org/virtio/virtio/v1.1/virtio-v1.1.pdf

use crate::bus::VIRTIO_BASE;

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
/// Physical page number for queue, read/write.
pub const VIRTIO_QUEUE_PFN: usize = VIRTIO_BASE + 0x040;

pub const VIRTIO_STATUS: usize = VIRTIO_BASE + 0x070;
/*
// used ring alignment, write-only
pub const VIRTIO_QUEUE_ALIGN: usize = 0x03c;
// ready bit
pub const VIRTIO_QUEUE_READY: usize = 0x044;
// write-only
pub const VIRTIO_QUEUE_NOTIFY: usize = 0x050;
// read-only
pub const VIRTIO_INTERRUPT_STATUS: usize = 0x060;
// write-only
pub const VIRTIO_INTERRUPT_ACK: usize = 0x064;
// read/write
pub const VIRTIO_STATUS: usize = 0x070;
*/

/// Paravirtualized drivers for IO virtualization.
pub struct Virtio {
    driver_features: u32,
    page_size: u32,
    queue_sel: u32,
    queue_num: u32,
    queue_pfn: u32,
    status: u32,
}

impl Virtio {
    pub fn new() -> Self {
        Self {
            driver_features: 0,
            page_size: 0,
            queue_sel: 0,
            queue_num: 0,
            queue_pfn: 0,
            status: 0,
        }
    }

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

    pub fn write(&mut self, addr: usize, val: u32) {
        match addr {
            VIRTIO_DEVICE_FEATURES => self.driver_features = val,
            VIRTIO_GUEST_PAGE_SIZE => self.page_size = val,
            VIRTIO_QUEUE_SEL => self.queue_sel = val,
            VIRTIO_QUEUE_NUM => self.queue_num = val,
            VIRTIO_QUEUE_PFN => self.queue_pfn = val,
            VIRTIO_STATUS => self.status = val,
            _ => {}
        }
    }
}
