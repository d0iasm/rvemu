mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
//#[cfg(feature = "wee_alloc")]
//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str); // window.alert()
    #[wasm_bindgen(module = "index")]
    fn render(s: &str); // TODO: ReferenceError: render is not defined
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

// TODO: better way?
impl Register {
    fn str(&self) -> String {
        match *self {
            Register::X0 => String::from("x0"),
            Register::X1 => String::from("x1"),
            Register::X2 => String::from("x2"),
            Register::X3 => String::from("x3"),
            Register::X4 => String::from("x4"),
            Register::X5 => String::from("x5"),
            Register::X6 => String::from("x6"),
            Register::X7 => String::from("x7"),
            Register::X8 => String::from("x8"),
            Register::X9 => String::from("x9"),
            Register::X10 => String::from("x10"),
            Register::X11 => String::from("x11"),
            Register::X12 => String::from("x12"),
            Register::X13 => String::from("x13"),
            Register::X14 => String::from("x14"),
            Register::X15 => String::from("x15"),
            Register::X16 => String::from("x16"),
            Register::X17 => String::from("x17"),
            Register::X18 => String::from("x18"),
            Register::X19 => String::from("x19"),
            Register::X20 => String::from("x20"),
            Register::X21 => String::from("x21"),
            Register::X22 => String::from("x22"),
            Register::X23 => String::from("x23"),
            Register::X24 => String::from("x24"),
            Register::X25 => String::from("x25"),
            Register::X26 => String::from("x26"),
            Register::X27 => String::from("x27"),
            Register::X28 => String::from("x28"),
            Register::X29 => String::from("x29"),
            Register::X30 => String::from("x30"),
            Register::X31 => String::from("x31"),
            Register::X32 => String::from("x32"),
            Register::XPC => String::from("pc"),
        }
    }
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
