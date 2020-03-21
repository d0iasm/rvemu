import init, { Emulator } from "./pkg/rvemu_wasm.js";

const deleteLine = "\x1b[2K\r";

const kernelReader = new FileReader();
const fsImgReader = new FileReader();
let fsImgData = null;

let emu = null;

function loadFiles() {
  fetch("./apps/fs.img")
    .then(response => response.blob())
    .then(blob => {
      const fsImgFile = new File([blob], "fs.img");
      fsImgReader.fileName = "fs.img";
      fsImgReader.readAsArrayBuffer(fsImgFile);

      fetch("./apps/xv6.text")
        .then(response => response.blob())
        .then(blob => {
          const kernelFile = new File([blob], "xv6");
          kernelReader.fileName = "xv6";
          kernelReader.readAsArrayBuffer(kernelFile);
        });
    });
}

async function initEmulator() {
  // Load the wasm file.
  await init();

  loadFiles();

  fsImgReader.onloadend = e => {
    fsImgData = new Uint8Array(fsImgReader.result);
    console.log("Loaded fs.img", fsImgData);
  };

  kernelReader.onloadend = e => {
    emu = Emulator.new();

    const kernelData = new Uint8Array(kernelReader.result);
    emu.set_dram(kernelData);
    emu.set_disk(fsImgData);

    console.log("Starting to execute xv6 ...");

    try {
      emu.start();
    } catch(err) {
      console.log(err);
    } finally {
      //emu.dump_registers();
      emu = null;
    }
  };
}

initEmulator();

const inputWorker = new Worker('input.js', {type: 'module'});
inputWorker.onmessage = e => {
  console.log("come!!!", e);
};

