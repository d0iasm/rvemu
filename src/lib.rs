mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
//#[cfg(feature = "wee_alloc")]
//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str); // window.alert()
    fn render(s: &str); // TODO: ReferenceError: render is not defined
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str); // console.log()
}

const REGISTERS_COUNT: usize = 33;

enum Register {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X31,
    X32,
    PC,
}

#[wasm_bindgen]
pub struct Emulator {
    registers: [u64; REGISTERS_COUNT],
    memory: Vec<u8>,
    pc: usize,
}

#[wasm_bindgen]
impl Emulator {
    pub fn new() -> Emulator {
        log("hoge");
        alert("hogehoge");
        render("hogehoge");
        return Emulator {
            registers: [0; REGISTERS_COUNT],
            memory: Vec::new(),
            pc: 0,
        };
    }

    pub fn set_binary(&mut self, text: String) {
        render("welcome to Risc-V Emulator.");
        self.memory = text.into_bytes();
    }

    pub fn dump_registers(&self) -> String {
        String::from("pc: ") + &self.registers[32].to_string()
        //for i in 0..REGISTERS_COUNT {
            //println!("{0} = {1}", REGISTERS_NAME[i], get_register32(emu, i));
        //}
    }

    pub fn exec(&mut self) {
        let size = self.memory.len();

        while self.pc < size {

        }
    }
}

type InstFunc = fn(&mut Emulator);
type Insts = [InstFunc; 256];

pub fn nop(emu: &mut Emulator) {
    emu.pc += 1;
}

pub fn undefined(_emu: &mut Emulator) {
}

pub fn init_instructions(instructions: &mut Insts) {
    instructions[0x01] = nop;
}
