import { Emulator } from "riscv-emu";

const screen = document.getElementById("screen");
const execBtn = document.getElementById("exec");
const fileIn = document.getElementById("file");
const stateDiv = document.getElementById("state");

const emu = Emulator.new();
const fileReader = new FileReader();
let execute_once = false;

fileReader.onloadend = e => {
  emu.set_binary(fileReader.result);
  emu.execute();
  execute_once = true;
};

fileIn.onchange = e => {
  emu.reset();
  execute_once = false;
  while (stateDiv.hasChildNodes()) {
    stateDiv.removeChild(stateDiv.firstChild);
  }
}

execBtn.onclick = e => {
  if (execute_once) {
    emu.reset();
    emu.execute();
    return;
  }

  const text = document.createTextNode("Set the file (" + fileIn.files[0].name + ")");
  stateDiv.appendChild(text);
  fileReader.readAsText(file.files[0]);
};
