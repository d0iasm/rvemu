//! The virtio module contains a virtualization standard for network and disk device drivers.
//! This is the "legacy" virtio interface.
//!
//! The virtio spec:
//! https://docs.oasis-open.org/virtio/virtio/v1.1/virtio-v1.1.pdf

use crate::bus::VIRTIO_BASE;
use crate::cpu::Cpu;
use crate::exception::Exception;

/// The interrupt request of virtio.
pub const VIRTIO_IRQ: u64 = 1;

/// The size of `VRingDesc` struct.
const VRING_DESC_SIZE: u64 = 16;
/// The number of virtio descriptors. It must be a power of two.
const DESC_NUM: u64 = 8;

/// Always return 0x74726976.
pub const VIRTIO_MAGIC: u64 = VIRTIO_BASE + 0x000;
/// The version. 1 is legacy.
pub const VIRTIO_VERSION: u64 = VIRTIO_BASE + 0x004;
/// device type; 1 is net, 2 is disk.
pub const VIRTIO_DEVICE_ID: u64 = VIRTIO_BASE + 0x008;
/// Always return 0x554d4551
pub const VIRTIO_VENDOR_ID: u64 = VIRTIO_BASE + 0x00c;
/// Device features.
/// https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html#x1-150002
pub const VIRTIO_DEVICE_FEATURES: u64 = VIRTIO_BASE + 0x010;
/// Driver features.
/// https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html#x1-140001
pub const VIRTIO_DRIVER_FEATURES: u64 = VIRTIO_BASE + 0x020;
/// Page size for PFN, write-only.
pub const VIRTIO_GUEST_PAGE_SIZE: u64 = VIRTIO_BASE + 0x028;
/// Select queue, write-only.
pub const VIRTIO_QUEUE_SEL: u64 = VIRTIO_BASE + 0x030;
/// Max size of current queue, read-only. In QEMU, `VIRTIO_COUNT = 8`.
pub const VIRTIO_QUEUE_NUM_MAX: u64 = VIRTIO_BASE + 0x034;
/// Size of current queue, write-only.
pub const VIRTIO_QUEUE_NUM: u64 = VIRTIO_BASE + 0x038;
/// Physical page number for queue, read and write.
pub const VIRTIO_QUEUE_PFN: u64 = VIRTIO_BASE + 0x040;
/// Notify the queue number, write-only.
pub const VIRTIO_QUEUE_NOTIFY: u64 = VIRTIO_BASE + 0x050;
/// Device status, read and write. Reading from this register returns the current device status flags.
/// Writing non-zero values to this register sets the status flags, indicating the OS/driver
/// progress. Writing zero (0x0) to this register triggers a device reset.
pub const VIRTIO_STATUS: u64 = VIRTIO_BASE + 0x070;

/// "The descriptor table refers to the buffers the driver is using for the device. addr is a
/// physical address, and the buffers can be chained via next. Each descriptor describes a buffer
/// which is read-only for the device (“device-readable”) or write-only for the device
/// (“device-writable”), but a chain of descriptors can contain both device-readable and
/// device-writable buffers."
///
/// https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html#x1-320005
///
/// ```c
/// struct virtq_desc {
///   /* Address (guest-physical). */
///   le64 addr;
///   /* Length. */
///   le32 len;
///
///   /* This marks a buffer as continuing via the next field. */
///   #define VIRTQ_DESC_F_NEXT   1
///   /* This marks a buffer as device write-only (otherwise device read-only). */
///   #define VIRTQ_DESC_F_WRITE     2
///   /* This means the buffer contains a list of buffer descriptors. */
///   #define VIRTQ_DESC_F_INDIRECT   4
///   /* The flags as indicated above. */
///   le16 flags;
///   /* Next field if flags & NEXT */
///   le16 next;
/// };
/// ```
struct VirtqDesc {
    /// 64-bit address.
    addr: u64,
    /// 32-bit length.
    len: u64,
    /// 16-bit flags.
    flags: u64,
    /// 16-bit next.
    next: u64,
}

impl VirtqDesc {
    fn new(cpu: &Cpu, addr: u64) -> Result<Self, Exception> {
        Ok(Self {
            addr: cpu.bus.read64(addr)?,
            len: cpu.bus.read32(addr.wrapping_add(8))?,
            flags: cpu.bus.read16(addr.wrapping_add(12))?,
            next: cpu.bus.read16(addr.wrapping_add(14))?,
        })
    }
}

/// Paravirtualized drivers for IO virtualization.
pub struct Virtio {
    id: u64,
    /// Each virtio device offers all the features it understands.
    /// https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html#x1-130002
    /// 0 to 23: Feature bits for the specific device type
    /// 24 to 40: Feature bits reserved for extensions to the queue and
    ///           feature negotiation mechanisms
    /// 41 to 63: Feature bits reserved for future extensions
    driver_features: u32,
    page_size: u32,
    queue_sel: u32,
    queue_num: u32,
    queue_pfn: u32,
    queue_notify: u32,
    /// "The device status field provides a simple low-level indication of the completed steps of
    /// this sequence.
    /// The device MUST initialize device status to 0 upon reset."
    /// https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html#x1-100001
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
    pub fn read(&self, addr: u64) -> u32 {
        match addr {
            VIRTIO_MAGIC => 0x74726976,     // read-only
            VIRTIO_VERSION => 0x1,          // read-only
            VIRTIO_DEVICE_ID => 0x2,        // read-only
            VIRTIO_VENDOR_ID => 0x554d4551, // read-only
            VIRTIO_DEVICE_FEATURES => 0,    // TODO: what should it return?
            VIRTIO_DRIVER_FEATURES => self.driver_features,
            VIRTIO_QUEUE_NUM_MAX => 8,
            VIRTIO_QUEUE_PFN => self.queue_pfn,
            VIRTIO_STATUS => self.status,
            _ => 0,
        }
    }

    /// Write 4 bytes to virtio only if the addr is valid. Otherwise, does nothing.
    pub fn write(&mut self, addr: u64, val: u32) {
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

    fn get_new_id(&mut self) -> u64 {
        self.id = self.id.wrapping_add(1);
        self.id
    }

    fn desc_addr(&self) -> u64 {
        self.queue_pfn as u64 * self.page_size as u64
    }

    fn read_disk(&self, addr: u64) -> u64 {
        self.disk[addr as usize] as u64
    }

    fn write_disk(&mut self, addr: u64, value: u64) {
        self.disk[addr as usize] = value as u8
    }

    /// Access the disk via virtio. This is an associated function which takes a `cpu` object to
    /// read and write with a memory directly (DMA).
    pub fn disk_access(cpu: &mut Cpu) -> Result<(), Exception> {
        // https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html#x1-230005
        // "Each virtqueue can consist of up to 3 parts:
        //     Descriptor Area - used for describing buffers
        //     Driver Area - extra data supplied by driver to the device
        //     Device Area - extra data supplied by device to driver"
        //
        // https://github.com/mit-pdos/xv6-riscv/blob/riscv/kernel/virtio_disk.c#L101-L103
        //     desc = pages -- num * VirtqDesc
        //     avail = pages + 0x40 -- 2 * uint16, then num * uint16
        //     used = pages + 4096 -- 2 * uint16, then num * vRingUsedElem
        let desc_addr = cpu.bus.virtio.desc_addr();
        let avail_addr = cpu.bus.virtio.desc_addr() + 0x40;
        let used_addr = cpu.bus.virtio.desc_addr() + 4096;

        // avail[0] is flags
        // avail[1] tells the device how far to look in avail[2...].
        let offset = cpu.bus.read16(avail_addr.wrapping_add(1))?;
        // avail[2...] are desc[] indices the device should process.
        // we only tell device the first index in our chain of descriptors.
        let index = cpu
            .bus
            .read16(avail_addr.wrapping_add(offset % DESC_NUM).wrapping_add(2))?;

        // First descriptor.
        let desc0 = VirtqDesc::new(cpu, desc_addr + VRING_DESC_SIZE * index)?;

        // Second descriptor.
        let desc1 = VirtqDesc::new(cpu, desc_addr + VRING_DESC_SIZE * desc0.next)?;

        // Read `virtio_blk_outhdr`. Add 8 because of its structure.
        // struct virtio_blk_outhdr {
        //   uint32 type;
        //   uint32 reserved;
        //   uint64 sector;
        // } buf0;
        let blk_sector = cpu.bus.read64(desc0.addr.wrapping_add(8))?;

        // Write to a device if the second bit of `flags` is set.
        match (desc1.flags & 2) == 0 {
            true => {
                // Read memory data and write it to a disk directly (DMA).
                for i in 0..desc1.len {
                    let data = cpu.bus.read8(desc1.addr + i)?;
                    cpu.bus.virtio.write_disk(blk_sector * 512 + i, data);
                }
            }
            false => {
                // Read disk data and write it to memory directly (DMA).
                for i in 0..desc1.len {
                    let data = cpu.bus.virtio.read_disk(blk_sector * 512 + i);
                    cpu.bus.write8(desc1.addr + i, data)?;
                }
            }
        };

        // Write id to `UsedArea`. Add 2 because of its structure.
        // struct UsedArea {
        //   uint16 flags;
        //   uint16 id;
        //   struct VRingUsedElem elems[NUM];
        // };
        let new_id = cpu.bus.virtio.get_new_id();
        cpu.bus.write16(used_addr.wrapping_add(2), new_id % 8)?;
        Ok(())
    }
}
