import { Emulator } from "riscv-emu";
import { Terminal } from "xterm";
import { FitAddon } from "xterm-addon-fit";

const execBtn = document.getElementById("exec");
const fileIn = document.getElementById("file");

const termContainer = document.getElementById("terminal");
const term = new Terminal({cursorBlink: true});
const fitAddon = new FitAddon();
const newLine = "\r\n$ ";
const deleteLine = "\x1b[2K\r";

term.loadAddon(fitAddon);
term.open(termContainer);
fitAddon.fit();

const emu = Emulator.new();
const fileReader = new FileReader();
let execute_once = false;

let files = [];

runTerminal();

fileReader.onloadend = e => {
  const bin = new Uint8Array(fileReader.result);
  emu.set_binary(bin);
  emu.execute();
};

fileIn.onchange = e => {
  let names = "";
  for (let i=0; i<file.files.length; i++) {
    names += file.files[i].name;
    if (i < file.files.length-1) {
      names += ", ";
    }
  }
  files.push.apply(files, file.files);
  term.write(deleteLine);
  term.write("uploaded " + names);
  prompt();
};

execBtn.onclick = e => {
  fileReader.readAsArrayBuffer(file.files[0]);
};

function help() {
  term.writeln('Supports the following commands:');
  term.writeln('  upload      open a local file to execute on the emulator');
  term.writeln('  ls          list files you uploaded');
  term.writeln('  run [file]  execute a file');
  term.write('  help        print all commands you can use');
}

function prompt() {
  term.write(newLine);
}

function upload() {
  fileIn.click();
}

function run(filename) {
  for (let i=0; i<files.length; i++) {
    if (filename == files[i].name) {
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
      names += ", ";
    }
  }
  term.write("\r\n" + names);
}

function command(input) {
  const com = input.split(" ");
  switch (com[0]) {
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
      term.write("\r\nrun " + com[1]);
      run(com[1]);
      break;
    default:
      term.writeln("");
      help();
  }
}

function runTerminal() {
  if (term._initialized) {
      return;
  }

  term._initialized = true;

  term.prompt = () => {
    term.write('\r\n$ ');
  };

  term.writeln('Welcome to RISC-V emulator');
  help();
  term.writeln('');
  prompt();

  let input = "";
  let cursor = 0;
  term.onKey(e => {
    const printable = !e.domEvent.altKey && !e.domEvent.altGraphKey && !e.domEvent.ctrlKey && !e.domEvent.metaKey;

    if (e.domEvent.keyCode === 13) { // Enter key
      command(input);
      input = "";
      cursor = 0;
      prompt();
    } else if (e.domEvent.keyCode === 8) { // Backspace key
      // Do not delete the prompt
      if (term._core.buffer.x > 2) {
        input = input.substr(0, cursor-1) + input.substr(cursor);
        if (cursor > input.length) {
          cursor--;
        }
        term.write(deleteLine);
        term.write("$ ");
        term.write(input);
      }
    } else if (e.domEvent.keyCode === 37) { // Arrow left
      if (cursor > 0) {
        cursor--;
      }
      term.write(e.key);
    } else if (e.domEvent.keyCode === 39) { // Arrow right
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
