mod cpu;
mod utils;

use crate::cpu::register::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
//#[cfg(feature = "wee_alloc")]
//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Emulator {
    registers: [u64; REGISTERS_COUNT],
    memory: Vec<u8>,
    pc: usize,
}

#[wasm_bindgen]
impl Emulator {
    pub fn new() -> Emulator {
        // Initialize for putting error messages to console.error.
        utils::set_panic_hook();

        return Emulator {
            registers: [0; REGISTERS_COUNT],
            memory: Vec::new(),
            pc: 0,
        };
    }

    pub fn set_binary(&mut self, text: String) {
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

    pub fn render(&self, content: &str) {
        let window = web_sys::window()
            .expect("no global `window` exists");
        let document = window.document()
            .expect("should have a document on window");
        let screen = document.get_element_by_id("screen")
            .expect("should have a element with a `screen` id");

        let div = document.create_element("div")
            .expect("div element should be created successfully");
        div.set_inner_html(content);
        let result = screen.append_child(&div);
        if result.is_err() {
            panic!("can't append a div node to a screen node")
        }

        let maxline = 51;
        if screen.child_element_count() > maxline {
            let child = screen.first_element_child()
                .expect("screen should have at least 1 child");
            let result = screen.remove_child(&child);
            if result.is_err() {
                panic!("can't remove a first div node from a screen node")
            }
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
