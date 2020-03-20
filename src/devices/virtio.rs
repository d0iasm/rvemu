//! The virtio module contains a virtualization standard for network and disk device drivers.
//! This is the "legacy" virtio interface.
//!
//! The virtio spec:
//! https://docs.oasis-open.org/virtio/virtio/v1.1/virtio-v1.1.pdf

use crate::bus::VIRTIO_BASE;
use crate::cpu::Cpu;

/// The interrupt request of virtio.
pub const VIRTIO_IRQ: usize = 1;

/// The size of `VRingDesc` struct in xv6.
/// ```c
/// struct VRingDesc {
///   uint64 addr;
///   uint32 len;
///   uint16 flags;
///   uint16 next;
/// };
/// ```
const VRING_DESC_SIZE: usize = 16;
/// The number of virtio descriptors. It must be a power of two.
const DESC_NUM: usize = 8;

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

    /// Read 4 bytes from virtio only if the addr is valid. Otherwise, return 0.
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

    /// Write 4 bytes to virtio only if the addr is valid. Otherwise, does nothing.
    pub fn write(&mut self, addr: usize, val: u32) {
        match addr {
            VIRTIO_DEVICE_FEATURES => self.driver_features = val,
            VIRTIO_GUEST_PAGE_SIZE => self.page_size = val,
            VIRTIO_QUEUE_SEL => self.queue_sel = val,
            VIRTIO_QUEUE_NUM => self.queue_num = val,
            VIRTIO_QUEUE_PFN => self.queue_pfn = val,
            VIRTIO_QUEUE_NOTIFY => self.queue_notify = val,
            VIRTIO_STATUS => self.status = val,
            _ => {}
        }
    }

    fn get_new_id(&mut self) -> u8 {
        self.id = self.id.wrapping_add(1);
        self.id
    }

    fn desc_addr(&self) -> u64 {
        self.queue_pfn as u64 * self.page_size as u64
    }

    fn read_disk(&self, addr: u64) -> u8 {
        self.disk[addr as usize]
    }

    fn write_disk(&mut self, addr: u64, value: u8) {
        self.disk[addr as usize] = value
    }

    /// Access the disk via virtio. This is an associated function which takes a `cpu` object to
    /// read and write with a memory directly (DMA).
    pub fn disk_access(cpu: &mut Cpu) {
        // See more information in
        // https://github.com/mit-pdos/xv6-riscv/blob/riscv/kernel/virtio_disk.c

        // the spec says that legacy block operations use three
        // descriptors: one for type/reserved/sector, one for
        // the data, one for a 1-byte status result.

        // desc = pages -- num * VRingDesc
        // avail = pages + 0x40 -- 2 * uint16, then num * uint16
        // used = pages + 4096 -- 2 * uint16, then num * vRingUsedElem
        let desc_addr = cpu.bus.virtio.desc_addr() as usize;
        let avail_addr = (cpu.bus.virtio.desc_addr() + 0x40) as usize;
        let used_addr = (cpu.bus.virtio.desc_addr() + 4096) as usize;

        // avail[0] is flags
        // avail[1] tells the device how far to look in avail[2...].
        let offset = cpu
            .bus
            .read16(avail_addr.wrapping_add(1))
            .expect("failed to read offset") as usize;
        // avail[2...] are desc[] indices the device should process.
        // we only tell device the first index in our chain of descriptors.
        let index = cpu
            .bus
            .read16(avail_addr.wrapping_add(offset % DESC_NUM).wrapping_add(2))
            .expect("failed to read index");

        // Read `VRingDesc`, virtio descriptors.
        let desc_addr0 = desc_addr + VRING_DESC_SIZE * index as usize;
        let addr0 = cpu
            .bus
            .read64(desc_addr0)
            .expect("failed to read an address field in a descriptor");
        // Add 14 because of `VRingDesc` structure.
        // struct VRingDesc {
        //   uint64 addr;
        //   uint32 len;
        //   uint16 flags;
        //   uint16 next
        // };
        // The `next` field can be accessed by offset 14 (8 + 4 + 2) bytes.
        let next0 = cpu
            .bus
            .read16(desc_addr0.wrapping_add(14))
            .expect("failed to read a next field in a descripor");

        // Read `VRingDesc` again, virtio descriptors.
        let desc_addr1 = desc_addr + VRING_DESC_SIZE * next0 as usize;
        let addr1 = cpu
            .bus
            .read64(desc_addr1)
            .expect("failed to read an address field in a descriptor");
        let len1 = cpu
            .bus
            .read32(desc_addr1.wrapping_add(8))
            .expect("failed to read a length field in a descriptor");
        let flags1 = cpu
            .bus
            .read16(desc_addr1.wrapping_add(12))
            .expect("failed to read a flags field in a descriptor");

        // Read `virtio_blk_outhdr`. Add 8 because of its structure.
        // struct virtio_blk_outhdr {
        //   uint32 type;
        //   uint32 reserved;
        //   uint64 sector;
        // } buf0;
        let blk_sector = cpu
            .bus
            .read64(addr0.wrapping_add(8) as usize)
            .expect("failed to read a sector field in a virtio_blk_outhdr");

        // Write to a device if the second bit `flag1` is set.
        match (flags1 & 2) == 0 {
            true => {
                // Read memory data and write it to a disk directly (DMA).
                for i in 0..len1 as u64 {
                    let data = cpu
                        .bus
                        .read8((addr1 + i) as usize)
                        .expect("failed to read from memory");
                    cpu.bus.virtio.write_disk(blk_sector * 512 + i, data);
                }
            }
            false => {
                // Read disk data and write it to memory directly (DMA).
                for i in 0..len1 as u64 {
                    let data = cpu.bus.virtio.read_disk(blk_sector * 512 + i);
                    cpu.bus
                        .write8((addr1 + i) as usize, data)
                        .expect("failed to write to memory");
                }
            }
        };

        // Write id to `UsedArea`. Add 2 because of its structure.
        // struct UsedArea {
        //   uint16 flags;
        //   uint16 id;
        //   struct VRingUsedElem elems[NUM];
        // };
        let new_id = cpu.bus.virtio.get_new_id() as u16;
        cpu.bus
            .write16(used_addr.wrapping_add(2) as usize, new_id % 8)
            .expect("failed to write to memory");
    }
}
