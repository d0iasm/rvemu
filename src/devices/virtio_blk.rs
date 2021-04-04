//! The virtio_blk module implements a virtio block device.
//!
//! The spec for Virtual I/O Device (VIRTIO) Version 1.1:
//! https://docs.oasis-open.org/virtio/virtio/v1.1/virtio-v1.1.html
//! 5.2 Block Device:
//! https://docs.oasis-open.org/virtio/virtio/v1.1/cs01/virtio-v1.1-cs01.html#x1-2390002

use crate::bus::VIRTIO_BASE;
use crate::cpu::{Cpu, BYTE, DOUBLEWORD, HALFWORD, WORD};
use crate::exception::Exception;

/// The interrupt request of virtio.
pub const VIRTIO_IRQ: u64 = 1;

/// The size of `VRingDesc` struct.
const VRING_DESC_SIZE: u64 = 16;
/// The number of virtio descriptors. It must be a power of two.
const QUEUE_SIZE: u64 = 8;
/// The size of a sector.
const SECTOR_SIZE: u64 = 512;

/// This marks a buffer as continuing via the next field.
const VIRTQ_DESC_F_NEXT: u64 = 1;
/// This marks a buffer as device write-only (otherwise device read-only).
const VIRTQ_DESC_F_WRITE: u64 = 2;
/// This means the buffer contains a list of buffer descriptors.
const _VIRTQ_DESC_F_INDIRECT: u64 = 4;

// 4.2.2 MMIO Device Register Layout
// https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html#x1-1460002
/// Magic value. Always return 0x74726976 (a Little Endian equivalent of the "virt" string).
const MAGIC: u64 = VIRTIO_BASE;
const MAGIC_END: u64 = VIRTIO_BASE + 0x3;

/// Device version number. 1 is legacy.
const VERSION: u64 = VIRTIO_BASE + 0x4;
const VERSION_END: u64 = VIRTIO_BASE + 0x7;

/// Virtio Subsystem Device ID. 1 is network, 2 is block device.
const DEVICE_ID: u64 = VIRTIO_BASE + 0x8;
const DEVICE_ID_END: u64 = VIRTIO_BASE + 0xb;

/// Virtio Subsystem Vendor ID. Always return 0x554d4551
const VENDOR_ID: u64 = VIRTIO_BASE + 0xc;
const VENDOR_ID_END: u64 = VIRTIO_BASE + 0xf;

/// Flags representing features the device supports. Access to this register returns bits
/// DeviceFeaturesSel ∗ 32 to (DeviceFeaturesSel ∗ 32) + 31.
const DEVICE_FEATURES: u64 = VIRTIO_BASE + 0x10;
const DEVICE_FEATURES_END: u64 = VIRTIO_BASE + 0x13;

/// Device (host) features word selection.
const DEVICE_FEATURES_SEL: u64 = VIRTIO_BASE + 0x14;
const DEVICE_FEATURES_SEL_END: u64 = VIRTIO_BASE + 0x17;

/// Flags representing device features understood and activated by the driver. Access to this
/// register sets bits DriverFeaturesSel ∗ 32 to (DriverFeaturesSel ∗ 32) + 31.
const DRIVER_FEATURES: u64 = VIRTIO_BASE + 0x20;
const DRIVER_FEATURES_END: u64 = VIRTIO_BASE + 0x23;

/// Activated (guest) features word selection.
const DRIVER_FEATURES_SEL: u64 = VIRTIO_BASE + 0x24;
const DRIVER_FEATURES_SEL_END: u64 = VIRTIO_BASE + 0x27;

// 4.2.4 Legacy interface
// https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html#x1-1560004
/// Guest page size. The driver writes the guest page size in bytes to the register during
/// initialization, before any queues are used. This value should be a power of 2 and is used by
/// the device to calculate the Guest address of the first queue page. Write-only.
const GUEST_PAGE_SIZE: u64 = VIRTIO_BASE + 0x28;
const GUEST_PAGE_SIZE_END: u64 = VIRTIO_BASE + 0x2b;

/// Virtual queue index. Writing to this register selects the virtual queue that the following
/// operations on the QueueNumMax, QueueNum, QueueAlign and QueuePFN registers apply to. The index
/// number of the first queue is zero (0x0). Write-only.
const QUEUE_SEL: u64 = VIRTIO_BASE + 0x30;
const QUEUE_SEL_END: u64 = VIRTIO_BASE + 0x33;

/// Maximum virtual queue size. Reading from the register returns the maximum size of the queue the
/// device is ready to process or zero (0x0) if the queue is not available. This applies to the
/// queue selected by writing to QueueSel and is allowed only when QueuePFN is set to zero (0x0),
/// so when the queue is not actively used. Read-only. In QEMU, `VIRTIO_COUNT = 8`.
const QUEUE_NUM_MAX: u64 = VIRTIO_BASE + 0x34;
const QUEUE_NUM_MAX_END: u64 = VIRTIO_BASE + 0x37;

/// Virtual queue size. Queue size is the number of elements in the queue, therefore size of the
/// descriptor table and both available and used rings. Writing to this register notifies the
/// device what size of the queue the driver will use. This applies to the queue selected by
/// writing to QueueSel. Write-only.
const QUEUE_NUM: u64 = VIRTIO_BASE + 0x38;
const QUEUE_NUM_END: u64 = VIRTIO_BASE + 0x3b;

/// Used Ring alignment in the virtual queue.
const QUEUE_ALIGN: u64 = VIRTIO_BASE + 0x3c;
const QUEUE_ALIGN_END: u64 = VIRTIO_BASE + 0x3f;

/// Guest physical page number of the virtual queue. Writing to this register notifies the device
/// about location of the virtual queue in the Guest’s physical address space. This value is the
/// index number of a page starting with the queue Descriptor Table. Value zero (0x0) means
/// physical address zero (0x00000000) and is illegal. When the driver stops using the queue it
/// writes zero (0x0) to this register. Reading from this register returns the currently used page
/// number of the queue, therefore a value other than zero (0x0) means that the queue is in use.
/// Both read and write accesses apply to the queue selected by writing to QueueSel.
const QUEUE_PFN: u64 = VIRTIO_BASE + 0x40;
const QUEUE_PFN_END: u64 = VIRTIO_BASE + 0x43;

// 4.2.2 MMIO Device Register Layout
// https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html#x1-1460002
/// Queue notifier. Writing a queue index to this register notifies the device that there are new
/// buffers to process in the queue. Write-only.
const QUEUE_NOTIFY: u64 = VIRTIO_BASE + 0x50;
const QUEUE_NOTIFY_END: u64 = VIRTIO_BASE + 0x53;

/// Interrupt status. Reading from this register returns a bit mask of events that caused the
/// device interrupt to be asserted.
const INTERRUPT_STATUS: u64 = VIRTIO_BASE + 0x60;
const INTERRUPT_STATUS_END: u64 = VIRTIO_BASE + 0x63;

/// Interrupt acknowledge. Writing a value with bits set as defined in InterruptStatus to this
/// register notifies the device that events causing the interrupt have been handled.
const INTERRUPT_ACK: u64 = VIRTIO_BASE + 0x64;
const INTERRUPT_ACK_END: u64 = VIRTIO_BASE + 0x67;

/// Device status. Reading from this register returns the current device status flags. Writing
/// non-zero values to this register sets the status flags, indicating the driver progress. Writing
/// zero (0x0) to this register triggers a device reset.
const STATUS: u64 = VIRTIO_BASE + 0x70;
const STATUS_END: u64 = VIRTIO_BASE + 0x73;

/// Configuration space.
const CONFIG: u64 = VIRTIO_BASE + 0x100;
const CONFIG_END: u64 = VIRTIO_BASE + 0x107;

/// https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html#x1-230005
/// "Each virtqueue can consist of up to 3 parts:
///     Descriptor Area - used for describing buffers
///     Driver Area - extra data supplied by driver to the device
///     Device Area - extra data supplied by device to driver"
/// "Note: Note that previous versions of this spec used different names for these parts
///     Descriptor Table - for the Descriptor Area
///     Available Ring - for the Driver Area
///     Used Ring - for the Device Area"
/// ```c
/// struct virtq {
///   struct virtq_desc desc[ Queue Size ];
///   struct virtq_avail avail;
///   u8 pad[ Padding ]; // Padding to the next Queue Align boundary.
///   struct virtq_used used;
/// };
/// ```
#[derive(Debug, Copy, Clone)]
struct VirtqueueAddr {
    /// The address that starts actual descriptors (16 bytes each).
    desc_addr: u64,
    /// The address that starts a ring of available descriptors.
    avail_addr: u64,
    /// The address that starts a ring of used descriptors.
    used_addr: u64,
}

impl VirtqueueAddr {
    /// Create a new virtqueue descriptor based on the address that stores the content of the
    /// descriptor.
    fn new(virtio: &Virtio) -> Self {
        // https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html#x1-240006
        // Virtqueue Part   | Alignment | Size
        // -------------------------------------------------
        // Descriptor Table | 16        | 16∗(Queue Size)
        // Available Ring   | 2         | 6 + 2∗(Queue Size)
        // Used Ring        | 4         | 6 + 8∗(Queue Size)

        let base_addr = virtio.queue_pfn as u64 * virtio.guest_page_size as u64;
        let align = virtio.queue_align as u64;
        let size = virtio.queue_num as u64;
        let avail_ring_end = base_addr + (16 * size) + (6 + 2 * size);

        Self {
            desc_addr: base_addr,
            avail_addr: base_addr + 16 * size,
            // Used ring starts with the `queue_align` boundary after the available ring ends.
            used_addr: (avail_ring_end.wrapping_div(align) + 1).wrapping_mul(align),
        }
    }
}

/// "The descriptor table refers to the buffers the driver is using for the device. addr is a
/// physical address, and the buffers can be chained via next. Each descriptor describes a buffer
/// which is read-only for the device (“device-readable”) or write-only for the device
/// (“device-writable”), but a chain of descriptors can contain both device-readable and
/// device-writable buffers."
///
/// https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html#x1-320005
///
/// ```c
/// /* This marks a buffer as continuing via the next field. */
/// #define VIRTQ_DESC_F_NEXT 1
/// /* This marks a buffer as device write-only (otherwise device read-only). */
/// #define VIRTQ_DESC_F_WRITE 2
/// /* This means the buffer contains a list of buffer descriptors. */
/// #define VIRTQ_DESC_F_INDIRECT 4
///
/// struct virtq_desc {
///   le64 addr;
///   le32 len;
///   le16 flags;
///   le16 next;
/// };
/// ```
#[derive(Debug)]
struct VirtqDesc {
    /// Address (guest-physical).
    addr: u64,
    /// Length.
    len: u64,
    /// The flags as indicated VIRTQ_DESC_F_NEXT/VIRTQ_DESC_F_WRITE/VIRTQ_DESC_F_INDIRECT.
    flags: u64,
    /// Next field if flags & NEXT.
    next: u64,
}

impl VirtqDesc {
    /// Creates a new virtqueue descriptor based on the address that stores the content of the
    /// descriptor.
    fn new(cpu: &mut Cpu, addr: u64) -> Result<Self, Exception> {
        Ok(Self {
            addr: cpu.bus.read(addr, DOUBLEWORD)?,
            len: cpu.bus.read(addr.wrapping_add(8), WORD)?,
            flags: cpu.bus.read(addr.wrapping_add(12), HALFWORD)?,
            next: cpu.bus.read(addr.wrapping_add(14), HALFWORD)?,
        })
    }
}

/// "The driver uses the available ring to offer buffers to the device: each ring entry refers to
/// the head of a descriptor chain. It is only written by the driver and read by the device."
///
/// https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html#x1-380006
///
/// ```c
/// #define VIRTQ_AVAIL_F_NO_INTERRUPT 1
/// struct virtq_avail {
///   le16 flags;
///   le16 idx;
///   le16 ring[ /* Queue Size */ ];
///   le16 used_event; /* Only if VIRTIO_F_EVENT_IDX */
/// };
/// ```
#[derive(Debug)]
struct VirtqAvail {
    flags: u16,
    idx: u16,
    ring_start_addr: u64,
}

impl VirtqAvail {
    fn new(cpu: &mut Cpu, addr: u64) -> Result<Self, Exception> {
        Ok(Self {
            flags: cpu.bus.read(addr, HALFWORD)? as u16,
            idx: cpu.bus.read(addr.wrapping_add(2), HALFWORD)? as u16,
            ring_start_addr: addr.wrapping_add(4),
        })
    }
}

/// Paravirtualized drivers for IO virtualization.
pub struct Virtio {
    id: u64,
    device_features: [u32; 2],
    device_features_sel: u32,
    driver_features: [u32; 2],
    driver_features_sel: u32,
    guest_page_size: u32,
    queue_num: u32,
    queue_align: u32,
    queue_pfn: u32,
    queue_notify: u32,
    interrupt_status: u32,
    status: u32,
    config: [u8; 8],
    disk: Vec<u8>,
    virtqueue: Option<VirtqueueAddr>,
}

impl Virtio {
    /// Creates a new virtio object.
    pub fn new() -> Self {
        let mut config = [0; 8];
        // https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html#x1-2440004
        // 5.2.4 Device configuration layout
        // struct virtio_blk_config {
        //   le64 capacity;
        // }
        //
        // The value is based on QEMU's output:
        // "virtio_blk virtio0: [vda] 204800 512-byte logical blocks (105 MB/100 MiB)"
        // 204800 --> 0x32000
        config[1] = 0x20;
        config[2] = 0x03;

        Self {
            id: 0,
            device_features: Virtio::device_features(),
            device_features_sel: 0,
            driver_features: [0; 2],
            driver_features_sel: 0,
            guest_page_size: 0,
            queue_num: 0,
            // default value to avoid division by 0.
            queue_align: 0x1000,
            queue_pfn: 0,
            queue_notify: u32::MAX,
            interrupt_status: 0,
            status: 0,
            config,
            disk: Vec::new(),
            virtqueue: None,
        }
    }

    /// Returns device features.
    fn device_features() -> [u32; 2] {
        let mut features = [0; 2];
        // VIRTIO_F_IN_ORDER(Bit 35). This feature indicates that all buffers are used by the device
        // in the same order in which they have been made available.
        features[1] = features[1] | (1 << 3);
        return features;
    }

    /// Initializes a virtqueue once the device initialization is finished by setting the DRIVER_OK
    /// status bit (0x4).
    fn init_virtqueue(&mut self) {
        let queue = VirtqueueAddr::new(self);
        self.virtqueue = Some(queue);
    }

    /// Gets `VirtqueueAddr` struct if it exists. If not, creates a new one based on the virtio
    /// configuration values.
    fn virtqueue(&self) -> VirtqueueAddr {
        match self.virtqueue {
            Some(queue) => queue,
            None => VirtqueueAddr::new(self),
        }
    }

    /// Resets the device when `status` is written to 0.
    fn reset(&mut self) {
        self.id = 0;
        // 4.2.2.1 Device Requirements: MMIO Device Register Layout
        // "Upon reset, the device MUST clear all bits in InterruptStatus and ready bits in the
        // QueueReady register for all queues in the device."
        self.interrupt_status = 0;
    }

    /// Returns true if an interrupt is pending.
    pub fn is_interrupting(&mut self) -> bool {
        if self.queue_notify != u32::MAX {
            self.queue_notify = u32::MAX;
            return true;
        }
        false
    }

    /// Sets the binary in the virtio disk.
    pub fn initialize(&mut self, binary: Vec<u8>) {
        self.disk.extend(binary.iter().cloned());
    }

    /// Loads `size`-bit data from a register located at `addr` in the virtio block device.
    pub fn read(&self, addr: u64, size: u8) -> Result<u64, Exception> {
        // `reg` is the value of a target register in the virtio block device and `offset` is the
        // byte of the start position in the register.
        let (reg, offset) = match addr {
            // A Little Endian equivalent of the “virt” string.
            MAGIC..=MAGIC_END => (0x74726976, addr - MAGIC),
            // Legacy devices (see 4.2.4 Legacy interface) used 0x1.
            VERSION..=VERSION_END => (0x1, addr - VERSION),
            // Block device.
            DEVICE_ID..=DEVICE_ID_END => (0x2, addr - DEVICE_ID),
            // See https://github.com/mit-pdos/xv6-riscv/blob/riscv/kernel/virtio_disk.c#L86
            VENDOR_ID..=VENDOR_ID_END => (0x554d4551, addr - VENDOR_ID),
            DEVICE_FEATURES..=DEVICE_FEATURES_END => (
                self.device_features[self.device_features_sel as usize],
                addr - DEVICE_FEATURES,
            ),
            QUEUE_NUM_MAX..=QUEUE_NUM_MAX_END => (QUEUE_SIZE as u32, addr - QUEUE_NUM_MAX),
            QUEUE_PFN..=QUEUE_PFN_END => (self.queue_pfn, addr - QUEUE_PFN),
            INTERRUPT_STATUS..=INTERRUPT_STATUS_END => {
                (self.interrupt_status, addr - INTERRUPT_STATUS)
            }
            STATUS..=STATUS_END => (self.status, addr - STATUS),
            CONFIG..=CONFIG_END => {
                if size != BYTE {
                    return Err(Exception::StoreAMOAccessFault);
                }
                let index = addr - CONFIG;
                (self.config[index as usize] as u32, 0)
            }
            _ => return Err(Exception::LoadAccessFault),
        };

        let value = match size {
            BYTE => (reg >> (offset * 8)) & 0xff,
            HALFWORD => (reg >> (offset * 8)) & 0xffff,
            WORD => (reg >> (offset * 8)) & 0xffffffff,
            _ => return Err(Exception::LoadAccessFault),
        };

        Ok(value as u64)
    }

    /// Stores `size`-bit data to a register located at `addr` in the virtio block device.
    pub fn write(&mut self, addr: u64, value: u32, size: u8) -> Result<(), Exception> {
        // `reg` is the value of a target register in the virtio block device and `offset` is the
        // byte of the start position in the register.
        let (mut reg, offset) = match addr {
            DEVICE_FEATURES_SEL..=DEVICE_FEATURES_SEL_END => {
                (self.device_features_sel, addr - DEVICE_FEATURES_SEL)
            }
            DRIVER_FEATURES..=DRIVER_FEATURES_END => (
                self.driver_features[self.driver_features_sel as usize],
                addr - DRIVER_FEATURES,
            ),
            DRIVER_FEATURES_SEL..=DRIVER_FEATURES_SEL_END => {
                (self.driver_features_sel, addr - DRIVER_FEATURES_SEL)
            }
            GUEST_PAGE_SIZE..=GUEST_PAGE_SIZE_END => (self.guest_page_size, addr - GUEST_PAGE_SIZE),
            QUEUE_SEL..=QUEUE_SEL_END => {
                if value != 0 {
                    panic!("Multiple virtual queues are not supported.");
                }
                return Ok(());
            }
            QUEUE_NUM..=QUEUE_NUM_END => (self.queue_num, addr - QUEUE_NUM),
            QUEUE_ALIGN..=QUEUE_ALIGN_END => (self.queue_align, addr - QUEUE_ALIGN),
            QUEUE_PFN..=QUEUE_PFN_END => (self.queue_pfn, addr - QUEUE_PFN),
            QUEUE_NOTIFY..=QUEUE_NOTIFY_END => (self.queue_notify, addr - QUEUE_NOTIFY),
            INTERRUPT_ACK..=INTERRUPT_ACK_END => {
                (self.interrupt_status, addr - INTERRUPT_ACK)
                /*
                if (value & 0x1) == 1 {
                    self.interrupt_status &= !0x1;
                } else {
                    panic!("unexpected value for INTERRUPT_ACK: {:#x}", value);
                }
                return Ok(());
                */
            }
            STATUS..=STATUS_END => (self.status, addr - STATUS),
            CONFIG..=CONFIG_END => {
                if size != BYTE {
                    return Err(Exception::StoreAMOAccessFault);
                }
                let index = addr - CONFIG;
                self.config[index as usize] = (value >> (index * 8)) as u8;
                return Ok(());
            }
            _ => return Err(Exception::StoreAMOAccessFault),
        };

        // Calculate the new value of the target register based on `size` and `offset`.
        match size {
            BYTE => {
                // Clear the target byte.
                reg = reg & (!(0xff << (offset * 8)));
                // Set the new `value` to the target byte.
                reg = reg | ((value & 0xff) << (offset * 8));
            }
            HALFWORD => {
                reg = reg & (!(0xffff << (offset * 8)));
                reg = reg | ((value & 0xffff) << (offset * 8));
            }
            WORD => {
                reg = value;
            }
            _ => return Err(Exception::StoreAMOAccessFault),
        }

        // Store the new register value to the target register.
        match addr {
            DEVICE_FEATURES_SEL..=DEVICE_FEATURES_SEL_END => self.device_features_sel = reg,
            DRIVER_FEATURES..=DRIVER_FEATURES_END => {
                self.driver_features[self.driver_features_sel as usize] = reg
            }
            DRIVER_FEATURES_SEL..=DRIVER_FEATURES_SEL_END => self.driver_features_sel = reg,
            GUEST_PAGE_SIZE..=GUEST_PAGE_SIZE_END => self.guest_page_size = reg,
            QUEUE_NUM..=QUEUE_NUM_END => self.queue_num = reg,
            QUEUE_ALIGN..=QUEUE_ALIGN_END => self.queue_align = reg,
            QUEUE_PFN..=QUEUE_PFN_END => self.queue_pfn = reg,
            QUEUE_NOTIFY..=QUEUE_NOTIFY_END => self.queue_notify = reg,
            INTERRUPT_ACK..=INTERRUPT_ACK_END => self.interrupt_status = reg,
            STATUS..=STATUS_END => {
                self.status = reg;
                // "Writing 0 into this field resets the device."
                if self.status == 0 {
                    self.reset();
                }
                // DRIVER_OK bit (4) was set, so initialize `VirtqueueAddr`.
                if self.status & 0x4 == 1 {
                    self.init_virtqueue();
                }
                // FAILED (128) bit. Indicates that something went wrong in the guest.
                if (self.status & 128) == 1 {
                    panic!("virtio: device status FAILED");
                }
            }
            _ => return Err(Exception::StoreAMOAccessFault),
        }

        Ok(())
    }

    fn read_disk(&self, addr: u64) -> u64 {
        self.disk[addr as usize] as u64
    }

    fn write_disk(&mut self, addr: u64, value: u64) {
        self.disk[addr as usize] = value as u8
    }

    /// Accesses the disk via virtio. This is an associated function which takes a `cpu` object to
    /// read and write with a memory directly (DMA).
    pub fn disk_access(cpu: &mut Cpu) -> Result<(), Exception> {
        // https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html#x1-1460002
        // "Used Buffer Notification
        //     - bit 0 - the interrupt was asserted because the device has used a buffer in at
        //     least one of the active virtual queues."
        cpu.bus.virtio.interrupt_status |= 0x1;

        let virtq = cpu.bus.virtio.virtqueue();

        let avail = VirtqAvail::new(cpu, virtq.avail_addr)?;

        let head_index = cpu.bus.read(
            avail.ring_start_addr + avail.idx as u64 % QUEUE_SIZE,
            HALFWORD,
        )?;

        // First descriptor.
        let desc0 = VirtqDesc::new(cpu, virtq.desc_addr + VRING_DESC_SIZE * head_index)?;
        assert_eq!(desc0.flags & VIRTQ_DESC_F_NEXT, 1);

        // Second descriptor.
        let desc1 = VirtqDesc::new(cpu, virtq.desc_addr + VRING_DESC_SIZE * desc0.next)?;
        assert_eq!(desc1.flags & VIRTQ_DESC_F_NEXT, 1);

        // 5.2.6 Device Operation
        // https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html#x1-2500006
        // struct virtio_blk_req {
        //   le32 type;
        //   le32 reserved;
        //   le64 sector;
        //   u8 data[][512];
        //   u8 status;
        // };
        let sector = cpu.bus.read(desc0.addr.wrapping_add(8), DOUBLEWORD)?;

        // Write to a device if the second bit of `flags` is set.
        match (desc1.flags & VIRTQ_DESC_F_WRITE) == 0 {
            true => {
                // Read memory data and write it to a disk.
                for i in 0..desc1.len {
                    let data = cpu.bus.read(desc1.addr + i, BYTE)?;
                    cpu.bus.virtio.write_disk(sector * SECTOR_SIZE + i, data);
                }
            }
            false => {
                // Read disk data and write it to memory.
                for i in 0..desc1.len {
                    let data = cpu.bus.virtio.read_disk(sector * SECTOR_SIZE + i);
                    cpu.bus.write(desc1.addr + i, data, BYTE)?;
                }
            }
        };

        // Third descriptor address.
        let desc2 = VirtqDesc::new(cpu, virtq.desc_addr + VRING_DESC_SIZE * desc1.next)?;
        assert_eq!(desc2.flags & VIRTQ_DESC_F_NEXT, 0);
        // Tell success.
        cpu.bus.write(desc2.addr, 0, BYTE)?;

        // 2.6.7.2 Device Requirements: Used Buffer Notification Suppression
        // https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html#x1-400007
        // After the device writes a descriptor index into the used ring:
        //   If flags is 1, the device SHOULD NOT send a notification.
        //   If flags is 0, the device MUST send a notification.
        // TODO: check the flags in the available ring.

        // "The used ring is where the device returns buffers once it is done with them: it is only
        // written to by the device, and read by the driver."
        //
        // https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html#x1-430008
        //
        // ```c
        // #define VIRTQ_USED_F_NO_NOTIFY 1
        // struct virtq_used {
        //   le16 flags;
        //   le16 idx;
        //   struct virtq_used_elem ring[ /* Queue Size */];
        //   le16 avail_event; /* Only if VIRTIO_F_EVENT_IDX */
        // };
        // ```
        cpu.bus.write(
            virtq
                .used_addr
                .wrapping_add(4)
                .wrapping_add((cpu.bus.virtio.id as u64 % QUEUE_SIZE) * 8),
            head_index,
            WORD,
        )?;

        cpu.bus.virtio.id = cpu.bus.virtio.id.wrapping_add(1);
        cpu.bus
            .write(virtq.used_addr.wrapping_add(2), cpu.bus.virtio.id, HALFWORD)?;

        Ok(())
    }
}
