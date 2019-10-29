import { Emulator } from "riscv-emu";

const screen = document.getElementById("screen");
const emu = Emulator.new();

// TODO: remove this block
for (let i=0; i<60; i++) {
  emu.render("hogehoge" + i);
}

const fileReader = new FileReader();
const execBtn = document.getElementById("exec");

fileReader.onloadend = e => {
  emu.set_binary(fileReader.result);
  emu.execute();
};

execBtn.onclick = e => {
  let file = document.getElementById("file");
  let state = document.getElementById("state");
  const text = document.createTextNode("Load file (" + file.files[0].name + ") ...");
  state.appendChild(text);
  fileReader.readAsText(file.files[0]);
};
