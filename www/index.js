import { Emulator } from "riscv-emu";

const screen = document.getElementById("screen");
const emu = Emulator.new();

const fileReader = new FileReader();
const execBtn = document.getElementById("exec");

fileReader.onloadend = e => {
  console.log(fileReader.result);
  emu.set_binary(fileReader.result);
};

execBtn.onclick = e => {
  let file = document.getElementById("file");
  let state = document.getElementById("state");
  const text = document.createTextNode("Load file (" + file.files[0].name + ") ...");
  state.appendChild(text);
  fileReader.readAsText(file.files[0]);
};

export function render(str) {
  console.log("called render" + str);
  const d = document.createElement("div");
  const t = document.createTextNode(str);
  d.appendChild(t);
  screen.appendChild(d);
}
