import { Emulator } from "riscv-emu";

const screen = document.getElementById("screen");
let ctx = screen.getContext("2d");

ctx.font = "8px Arial";
ctx.fillStyle = "#fff";
ctx.fillText("Hello World", 10, 30);

const emu = Emulator.new();
ctx.fillText(emu.dump_registers(), 10, 50);
ctx.fillText("hoge", 10, 70);
