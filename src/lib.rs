mod cpu;
mod utils;

use crate::cpu::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Emulator {
    cpu: Cpu,
    memory: Vec<u8>,
}

#[wasm_bindgen]
impl Emulator {
    pub fn new() -> Emulator {
        // Initialize for putting error messages to console.error.
        utils::set_panic_hook();

        Emulator {
            cpu: Cpu::new(),
            memory: Vec::new(),
        }
    }

    pub fn set_binary(&mut self, text: String) {
        self.memory = text.into_bytes();
    }

    pub fn dump_registers(&self) -> String {
        String::from("pc: ") + &self.cpu.registers[32].to_string()
        //for i in 0..REGISTERS_COUNT {
            //println!("{0} = {1}", REGISTERS_NAME[i], get_register32(emu, i));
        //}
    }

    pub fn execute(&mut self) {
        let size = self.memory.len();

        while self.cpu.pc < size {
            let code = self.cpu.fetch(&self.memory);
            self.cpu.execute(code, &mut self.memory);
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
