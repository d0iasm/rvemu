import init, { emulator_start } from "./pkg/rvemu_wasm.js";

const termContainer = document.getElementById("terminal");
const term  = new Terminal({cursorBlink: true});

const kernelReader = new FileReader();
const fsImgReader = new FileReader();

const fitAddon = new FitAddon.FitAddon();
const deleteLine = "\x1b[2K\r";

let inputBuffer = "";
const buffer = document.getElementById("buffer");

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
    console.log("Starting to execute xv6 ...");
    
    const kernelData = new Uint8Array(kernelReader.result);

    try {
      emulator_start(kernelData, fsImgData);
    } catch(err) {
      console.log(err);
    }
  };
}

function initTerminal() {
  term.loadAddon(fitAddon);
  term.open(termContainer);
  fitAddon.fit();

  if (term._initialized) {
      return;
  }

  term._initialized = true;

  term.writeln("Welcome to RISC-V emulator rvemu!");
  term.writeln("This is a work-in-progress project. You can see the progress at https://github.com/d0iasm/rvemu");
  term.writeln("Bug reports and feature requests are always welcome: https://github.com/d0iasm/rvemu/issues");
  term.writeln("");
  term.writeln("Loading operating system ...");

  let cursor = 0;
  term.onKey(e => {
    const printable = !e.domEvent.altKey && !e.domEvent.altGraphKey && !e.domEvent.ctrlKey && !e.domEvent.metaKey;
    console.log(printable, e.key, e.data, e);

    if (e.domEvent.code == 'Backspace') {
      if (buffer.childElementCount <= 0) {
          return;
      }
      buffer.removeChild(buffer.lastElementChild);
    } else if (e.domEvent.code == 'Enter') {
      const span = document.createElement('span');
      span.innerText = "\n";
      buffer.appendChild(span);
    } else if (printable) {
      const span = document.createElement('span');
      span.innerText = e.key;
      buffer.appendChild(span);
    }
  });
}

onmessage = e => {
  const c = String.fromCharCode(e.data);
  if (c == "\n") {
    term.writeln("");
  } else {
    term.write(c);
  }
}

initTerminal();
initEmulator();
