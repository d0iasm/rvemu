//! The dts modules contains a device tree source (DTS) and compile it to a device tree binary (DTB).

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
                #interrupt-cells = <1>;
                interrupt-controller;
                compatible = "riscv,cpu-intc";
                phandle = <0x2>;
            };
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
pub fn dtb() -> std::io::Result<Vec<u8>> {
    create_dts()?;
    compile_dts()?;

// TODO: set a reset vector correctly.
/*
    let reset_vec = vec![
        0x297,                // auipc  t0,0x0
        0x28593 + (32 << 20), // addi   a1, t0, &dtb
        0xf1402573,           // csrr   a0, mhartid
        0x0182b283,           // ld     t0,24(t0)
        0x28067,              // jr     t0
        0,
        (0x80000004 & 0xffffffff) as u32,
        (0x80000004 >> 32) as u32,
    ];
*/
	let mut dtb = Vec::new();
    File::open(DTB_FILE_NAME)?.read_to_end(&mut dtb)?;
    Ok(dtb)
}
