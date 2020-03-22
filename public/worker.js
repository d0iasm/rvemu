// static import for module script.
import init, { Emulator } from "./pkg/rvemu_wasm.js";

// dynamic import for module script.
//import('./pkg/rvemu_wasm.js').then(rvemu => {

const kernelReader = new FileReader();
const fsImgReader = new FileReader();

let emu = null;

function loadDisk() {
  return new Promise((resolve, reject) => {
    fsImgReader.onloadend = e => {
      console.log("Loaded fs.img");
      resolve(new Uint8Array(fsImgReader.result));
    };

    // Fetch fs.img.
    fetch("./apps/fs.img")
      .then(response => response.blob())
      .then(blob => {
        const fsImgFile = new File([blob], "fs.img");
        fsImgReader.fileName = "fs.img";
        fsImgReader.readAsArrayBuffer(fsImgFile);
      });
  });
}

async function initEmulator() {
  // Initialize for wasm.
  await init();

  const fsImgData = await loadDisk();

  // Fetch kernel image.
  fetch("./apps/xv6.text")
    .then(response => response.blob())
    .then(blob => {
      const kernelFile = new File([blob], "xv6");
      kernelReader.fileName = "xv6";
      kernelReader.readAsArrayBuffer(kernelFile);
    });

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
      emu.dump_registers();
      emu = null;
    }
  };
}

const inputWorker = new Worker('input.js', {type: 'module'});
inputWorker.onmessage = e => {
  postMessage(e.data);
};

initEmulator();
