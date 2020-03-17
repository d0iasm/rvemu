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
/// Physical page number for queue, read and write.
pub const VIRTIO_QUEUE_PFN: usize = VIRTIO_BASE + 0x040;
/// Device status, read and write. Reading from this register returns the current device status flags.
/// Writing non-zero values to this register sets the status flags, indicating the OS/driver
/// progress. Writing zero (0x0) to this register triggers a device reset.
pub const VIRTIO_STATUS: usize = VIRTIO_BASE + 0x070;

/// Paravirtualized drivers for IO virtualization.
pub struct Virtio {
    driver_features: u32,
    page_size: u32,
    queue_sel: u32,
    queue_num: u32,
    queue_pfn: u32,
    status: u32,
    disk: Vec<u8>,
    pub interrupting: bool,
}

impl Virtio {
    /// Create a new virtIO object.
    pub fn new() -> Self {
        Self {
            driver_features: 0,
            page_size: 0,
            queue_sel: 0,
            queue_num: 0,
            queue_pfn: 0,
            status: 0,
            disk: Vec::new(), 
            interrupting: false,
        }
    }

    /// Set the binary in the virtIO disk.
    pub fn set_disk(&mut self, binary: Vec<u8>) {
        self.disk.extend(binary.iter().cloned());
    }

    /// Read 4 bytes from virtIO only if the address is valid. Otherwise, return 0.
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

    /// Write 4 bytes to virtIO only if the address is valid. Otherwise, does nothing.
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
