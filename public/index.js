import init, { emulator_start } from "./pkg/rvemu_wasm.js";

const termContainer = document.getElementById("terminal");
const term  = new Terminal({cursorBlink: true});

const fitAddon = new FitAddon.FitAddon();
const newLine = "\r\n% ";
const deleteLine = "\x1b[2K\r";

let files = [];
const fileReader = new FileReader();
// This is the image file `fs.img` for xv6.
const fsImgReader = new FileReader();
let fsImgData = null;

// Files user uploaded.
const fileIn = document.getElementById("file");
// Input buffer detects user input while executing the emulator.
const inputBuffer = document.getElementById("inputBuffer");
// Output buffer detects the result of cpu state after the emulation is done.
const outputBuffer = document.getElementById("outputBuffer");
// Options for the observer (which mutations to observe)
const config = { childList: true, subtree: true };

const flag = document.getElementById("flag");

// Create an observer instance linked to the callback function which detect
// mutations.
const outputObserver = new MutationObserver((mutationsList, observer) => {
  for(let mutation of mutationsList) {
    if (mutation.type === 'childList' && mutation.addedNodes.length > 0) {
      term.write(deleteLine);
      const firstChild = mutation.addedNodes[0];
      const texts = firstChild.innerText.split("\n");
      for (let i=0; i<texts.length; i++) {
        term.writeln(texts[i]);
      }
      outputBuffer.removeChild(firstChild);
      term.write("% ");
    }
  }
});

async function initEmulator() {
  // Load the wasm file.
  await init();

  // Start observing the target node for configured mutations
  outputObserver.observe(outputBuffer, config);

  fileReader.onloadend = e => {
    const data = new Uint8Array(fileReader.result);

    try {
      // Set up fs.img for xv6.
      if (e.target.fileName == "xv6") {
        emulator_start(data, fsImgData);
        console.log("xv6 is executing...");
      } else {
        emulator_start(data, null);
      }
    } catch(err) {
      console.log("catch");
      term.write(deleteLine);
      term.write(err.message);
      prompt();
      console.log(err);
    }
  };

  fsImgReader.onloadend = e => {
    fsImgData = new Uint8Array(fsImgReader.result);
    console.log("set fs.img for xv6", fsImgData);
  };

  fileIn.onchange = e => {
    let names = "";
    for (let i=0; i<file.files.length; i++) {
      names += file.files[i].name;
      if (i < file.files.length-1) {
        names += "\t";
      }
    }
    files.push.apply(files, file.files);
    term.write(deleteLine);
    term.write("uploaded:\t" + names);
    prompt();
  };
}

function check_executing() {
  if (flag.textContent == 'active') {
    return true;
  }
  return false;
}

function prompt() {
  term.write(newLine);
}

function help() {
  term.writeln("Supports the following commands:");
  term.writeln("  upload      open local files for the execution on the emulator");
  term.writeln("  ls          list files you uploaded");
  term.writeln("  run [file]  execute a file");
  term.write("  help        print all commands you can use");
}

function upload() {
  fileIn.click();
}

function run(filename) {
  for (let i=0; i<files.length; i++) {
    if (filename == files[i].name) {
      fileReader.fileName = filename;
      fileReader.readAsArrayBuffer(files[i]);
      return;
    }
  }
  term.write("\r\n" + filename + ": No such file");
}

function ls() {
  let names = "";
  for (let i=0; i<files.length; i++) {
    names += files[i].name;
    if (i < files.length-1) {
      names += "\t";
    }
  }
  term.write("\r\n" + names);
}

function command(input) {
  const com = input.split(" ").filter(element => element.length != 0);
  switch (com[0]) {
    case "":
      break;
    case "upload":
      upload();
      break;
    case "ls":
      ls();
      break;
    case "run":
      if (com[1] == undefined) {
        term.writeln("");
        help();
        break;
      }
      run(com[1]);
      break;
    default:
      term.writeln("");
      help();
  }
}

function loadApps() {
  fetch("./apps/fib.text")
    .then(response => response.blob())
    .then(blob => {
      const sampleApp = new File([blob], "fib");
      files.push(sampleApp);
    });

  fetch("./apps/xv6.text")
    .then(response => response.blob())
    .then(blob => {
      const sampleApp = new File([blob], "xv6");
      files.push(sampleApp);
    });

  fetch("./apps/fs.img")
    .then(response => response.blob())
    .then(blob => {
      const fsImg = new File([blob], "fs.img");
      fsImgReader.fileName = "fs.img";
      fsImgReader.readAsArrayBuffer(fsImg);
    });
}

function initTerminal() {
  term.loadAddon(fitAddon);
  term.open(termContainer);
  fitAddon.fit();

  if (term._initialized) {
      return;
  }
  term._initialized = true;

  term.prompt = () => {
    term.write('\r\n% ');
  };

  term.writeln("Welcome to RISC-V emulator!");
  term.writeln("This is a work-in-progress project. You can see the progress at https://github.com/d0iasm/rvemu");
  term.writeln("Bug reports and feature requests are always welcome: https://github.com/d0iasm/rvemu/issues");
  term.writeln("");
  help();
  term.writeln("");
  prompt();

  loadApps();

  let input = "";
  let cursor = 0;
  term.onKey(e => {
    const printable = !e.domEvent.altKey && !e.domEvent.altGraphKey && !e.domEvent.ctrlKey && !e.domEvent.metaKey;

    if (check_executing()) {
      const span = document.createElement('span');
      if (e.key == "") {
        // Control characters (enter, backspace, etc.).
        span.innerText = e.domEvent.keyCode;
      } else {
        // Normal printable characters.
        span.innerText = e.key.charCodeAt(0);
      }
      inputBuffer.appendChild(span);
      return;
    }

    if (e.domEvent.code == 'Enter') {
      command(input);
      input = "";
      cursor = 0;
      prompt();
    } else if (e.domEvent.code == 'Backspace') {
      // Do not delete the prompt
      if (term._core.buffer.x > 2) {
        input = input.substr(0, cursor-1) + input.substr(cursor);
        if (cursor > input.length) {
          cursor--;
        }
        term.write(deleteLine);
        term.write("% ");
        term.write(input);
      }
    } else if (e.domEvent.code == 'ArrowLeft') {
      if (cursor > 0) {
        cursor--;
      }
      term.write(e.key);
    } else if (e.domEvent.code == 'ArrowRight') {
      if (cursor < input.length) {
        cursor++;
      }
      term.write(e.key);
    } else if (printable) {
      cursor++;
      input = input.substr(0, cursor) + e.key + input.substr(cursor);
      term.write(e.key);
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
