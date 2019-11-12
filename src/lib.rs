pub mod cpu;
mod utils;

use crate::cpu::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn render(content: &str) {
    let window = web_sys::window()
        .expect("no global `window` exists");
    let document = window.document()
        .expect("should have a document on window");
    let terminal = document.get_element_by_id("terminal")
        .expect("should have a element with a `terminal` id");

    let div = document.create_element("div")
        .expect("div element should be created successfully");
    div.set_inner_html(content);
    let result = terminal.append_child(&div);
    if result.is_err() {
        panic!("can't append a div node to a terminal node")
    }

    let maxline = 51;
    if terminal.child_element_count() > maxline {
        let child = terminal.first_element_child()
            .expect("terminal should have at least 1 child");
        let result = terminal.remove_child(&child);
        if result.is_err() {
            panic!("can't remove a first div node from a terminal node")
        }
    }
}

#[wasm_bindgen]
pub struct Emulator {
    cpu: Cpu,
    mem: Vec<u8>,
}

#[wasm_bindgen]
impl Emulator {
    pub fn new() -> Emulator {
        // Initialize for putting error messages to a browser console.
        utils::set_panic_hook();

        Emulator {
            cpu: Cpu::new(),
            mem: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.cpu.pc = 0;
        for i in 0..REGISTERS_COUNT {
            self.cpu.regs[i] = 0;
        }
    }

    pub fn set_binary(&mut self, bin: Vec<u8>) {
        self.mem = bin;
        log(&format!("binary size: {} ({:#x})", self.mem.len(), self.mem.len()));
    }

    pub fn execute(&mut self) {
        self.cpu.start(&mut self.mem);
        self.dump_registers();
    }

    pub fn dump_registers(&self) {
        for i in 0..REGISTERS_COUNT {
            let text = format!("x{}: {:#x} ({}, {:#b})",
                i, self.cpu.regs[i], self.cpu.regs[i], self.cpu.regs[i]);
            log(&text);
            render(&text);
        }

        log(&format!("pc: {}", self.cpu.pc));
        render(&format!("pc: {}", self.cpu.pc));
    }
}
