import { Emulator } from "riscv-emu";
import { Terminal } from "xterm";
import { FitAddon } from "xterm-addon-fit";

const execBtn = document.getElementById("exec");
const fileIn = document.getElementById("file");
const stateDiv = document.getElementById("state");

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

runTerminal();

fileReader.onloadend = e => {
  const bin = new Uint8Array(fileReader.result);
  emu.set_binary(bin);
  emu.execute();
  execute_once = true;
};

fileIn.onchange = e => {
  emu.reset();
  execute_once = false;
  while (stateDiv.hasChildNodes()) {
    stateDiv.removeChild(stateDiv.firstChild);
  }
};

execBtn.onclick = e => {
  if (execute_once) {
    emu.reset();
    emu.execute();
    return;
  }

  const text = document.createTextNode("Set the file (" + fileIn.files[0].name + ")");
  stateDiv.appendChild(text);
  fileReader.readAsArrayBuffer(file.files[0]);
};

function help(term) {
  term.writeln('Supports the following commands:');
  term.writeln('  upload      open a local file to execute on the emulator');
  term.writeln('  ls          list files you uploaded');
  term.writeln('  run [file]  execute a file');
  term.writeln('  help        print all commands you can use');
}

function prompt(term) {
  term.write(newLine);
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
  help(term);
  term.writeln('');
  prompt(term);

  let input = "";
  let cursor = 0;
  term.onKey(e => {
    const printable = !e.domEvent.altKey && !e.domEvent.altGraphKey && !e.domEvent.ctrlKey && !e.domEvent.metaKey;

    if (e.domEvent.keyCode === 13) { // Enter key
      console.log("Typed: " + input);
      input = "";
      cursor = 0;
      prompt(term);
    } else if (e.domEvent.keyCode === 8) { // Backspace key
      // Do not delete the prompt
      if (term._core.buffer.x > 2) {
        console.log(cursor);
        console.log(input);
        input = input.substr(0, cursor-1) + input.substr(cursor);
        cursor--;
        term.write(deleteLine);
        term.write("$ ");
        term.write(input);
        console.log(cursor);
        console.log(input);
      }
    } else if (e.domEvent.keyCode === 37) { // Arrow left
      if (cursor < input.length) {
        cursor++;
      }
      term.write(e.key);
      console.log(cursor);
    } else if (e.domEvent.keyCode === 39) { // Arrow right
      if (cursor > 0) {
        cursor--;
      }
      term.write(e.key);
      console.log(cursor);
    } else if (printable) {
      cursor++;
      input += e.key;
      console.log(cursor);
      term.write(e.key);
    }
  });
}
