//! The rom module contains the read-only memory structure and implementation to read the memory. ROM includes a device tree blob (DTB) compiled from a device tree source (DTS).

use crate::bus::MROM_BASE;

use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

const DTS_FILE_NAME: &str = "rvemu.dts";
const DTB_FILE_NAME: &str = "rvemu.dtb";

/// Create a new dts file. If the file already existed, the old content is destroyed. Otherwise, a new file is created.
fn create_dts() -> std::io::Result<()> {
    // TODO: Make this content more flexible depending on the number of cpus.
    // Reference code is https://github.com/riscv/riscv-isa-sim/blob/66b44bfbedda562a32e4a2cd0716afbf731b69cd/riscv/dts.cc#L38-L54
    let content = r#"/dts-v1/;

/ {
    #address-cells = <2>;
    #size-cells = <2>;
    compatible = "riscv-virtio";
    model = "riscv-virtio,qemu";

    chosen {
        bootargs = [00];
        stdout-path = "/uart@10000000";
    };

    uart@10000000 {
        interrupts = <0xa>;
        interrupt-parent = <0x2>;
        clock-frequency = <0x384000>;
        reg = <0x0 0x10000000 0x0 0x100>;
        compatible = "ns16550a";
    };

	virtio_mmio@10008000 {
		interrupts = <0x8>;
		interrupt-parent = <0x2>;
		reg = <0x0 0x10008000 0x0 0x1000>;
		compatible = "virtio,mmio";
	};

	virtio_mmio@10007000 {
		interrupts = <0x7>;
		interrupt-parent = <0x2>;
		reg = <0x0 0x10007000 0x0 0x1000>;
		compatible = "virtio,mmio";
	};

	virtio_mmio@10006000 {
		interrupts = <0x6>;
		interrupt-parent = <0x2>;
		reg = <0x0 0x10006000 0x0 0x1000>;
		compatible = "virtio,mmio";
	};

	virtio_mmio@10005000 {
		interrupts = <0x5>;
		interrupt-parent = <0x2>;
		reg = <0x0 0x10005000 0x0 0x1000>;
		compatible = "virtio,mmio";
	};

	virtio_mmio@10004000 {
		interrupts = <0x4>;
		interrupt-parent = <0x2>;
		reg = <0x0 0x10004000 0x0 0x1000>;
		compatible = "virtio,mmio";
	};

	virtio_mmio@10003000 {
		interrupts = <0x3>;
		interrupt-parent = <0x2>;
		reg = <0x0 0x10003000 0x0 0x1000>;
		compatible = "virtio,mmio";
	};

	virtio_mmio@10002000 {
		interrupts = <0x2>;
		interrupt-parent = <0x2>;
		reg = <0x0 0x10002000 0x0 0x1000>;
		compatible = "virtio,mmio";
	};

	virtio_mmio@10001000 {
		interrupts = <0x1>;
		interrupt-parent = <0x2>;
		reg = <0x0 0x10001000 0x0 0x1000>;
		compatible = "virtio,mmio";
	};

    cpus {
        #address-cells = <0x1>;
        #size-cells = <0x0>;
        timebase-frequency = <0x989680>;

        cpu@0 {
            device_type = "cpu";
            reg = <0x0>;
            status = "okay";
            riscv,isa = "rv64imafdcsu";
            mmu-type = "riscv,sv39";
            clock-frequency = <0x3b9aca00>;

            interrupt-controller {
                #interrupt-cells = <0x1>;
                interrupt-controller;
                compatible = "riscv,cpu-intc";
                linux,phandle = <0x1>;
                phandle = <0x1>;
            };
        };
    };

	memory@80000000 {
		device_type = "memory";
		reg = <0x0 0x80000000 0x0 0x8000000>;
	};

	soc {
		#address-cells = <0x2>;
		#size-cells = <0x2>;
		compatible = "riscv-virtio-soc";
		ranges;

		interrupt-controller@c000000 {
			linux,phandle = <0x2>;
			phandle = <0x2>;
			riscv,ndev = <0xa>;
			riscv,max-priority = <0x7>;
			reg-names = "control";
			reg = <0x0 0xc000000 0x0 0x4000000>;
			interrupts-extended = <0x1 0xb 0x1 0x9>;
			interrupt-controller;
			compatible = "riscv,plic0";
			#interrupt-cells = <0x1>;
		};

		clint@2000000 {
			interrupts-extended = <0x1 0x3 0x1 0x7>;
			reg = <0x0 0x2000000 0x0 0x10000>;
			compatible = "riscv,clint0";
		};
	};
};"#;

    let mut dts = File::create(DTS_FILE_NAME)?;
    dts.write_all(content.as_bytes())?;
    Ok(())
}

/// Compile a dts file to a dtb file.
fn compile_dts() -> std::io::Result<()> {
    // dtc -I dts -O dtb -o <FILE_NAME>.dtb <FILE_NAME>.dts
    Command::new("dtc")
        .args(&["-I", "dts", "-O", "dtb", "-o", DTB_FILE_NAME, DTS_FILE_NAME])
        .output()?;
    Ok(())
}

/// Read a dtb file. First, create a dts file. Second, compile it to a dtb file. Finally, read the dtb file and return the binary content.
fn dtb() -> std::io::Result<Vec<u8>> {
    create_dts()?;
    compile_dts()?;

    let mut dtb = Vec::new();
    File::open(DTB_FILE_NAME)?.read_to_end(&mut dtb)?;

    Ok(dtb)
}

/// The read-only memory (ROM).
pub struct Rom {
    data: Vec<u8>,
}

impl Rom {
    /// Create a new `rom` object.
    pub fn new() -> Self {
        let mut dtb = match dtb() {
            Ok(dtb) => dtb,
            Err(e) => {
                // TODO: should fail?
                println!("WARNING: failed to read a device tree binary: {}", e);
                println!(
                    "WARNING: maybe need to install dtc commend `apt install device-tree-compiler`"
                );
                Vec::new()
            }
        };

        // TODO: set a reset vector correctly.
        // 0x20 is the size of a reset vector.
        let mut rom = vec![0; 32];
        rom.append(&mut dtb);
        let align = 0x1000;
        rom.resize((rom.len() + align - 1) / align * align, 0);

        Self { data: rom }
    }

    /// Read a byte from the rom.
    pub fn read8(&self, addr: u64) -> u64 {
        let index = (addr - MROM_BASE) as usize;
        self.data[index] as u64
    }

    /// Read 2 bytes from the rom.
    pub fn read16(&self, addr: u64) -> u64 {
        let index = (addr - MROM_BASE) as usize;
        return (self.data[index] as u64) | ((self.data[index + 1] as u64) << 8);
    }

    /// Read 4 bytes from the rom.
    pub fn read32(&self, addr: u64) -> u64 {
        let index = (addr - MROM_BASE) as usize;
        return (self.data[index] as u64)
            | ((self.data[index + 1] as u64) << 8)
            | ((self.data[index + 2] as u64) << 16)
            | ((self.data[index + 3] as u64) << 24);
    }

    /// Read 8 bytes from the rom.
    pub fn read64(&self, addr: u64) -> u64 {
        let index = (addr - MROM_BASE) as usize;
        return (self.data[index] as u64)
            | ((self.data[index + 1] as u64) << 8)
            | ((self.data[index + 2] as u64) << 16)
            | ((self.data[index + 3] as u64) << 24)
            | ((self.data[index + 4] as u64) << 32)
            | ((self.data[index + 5] as u64) << 40)
            | ((self.data[index + 6] as u64) << 48)
            | ((self.data[index + 7] as u64) << 56);
    }
}
