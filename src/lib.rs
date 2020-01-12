pub mod cpu;
pub mod memory;
mod utils;

use crate::cpu::*;
use crate::memory::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn render(content: &str) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let buffer = document
        .get_element_by_id("buffer")
        .expect("should have a element with a `buffer` id");

    let span = document
        .create_element("span")
        .expect("span element should be created successfully");
    span.set_inner_html(content);
    let result = buffer.append_child(&span);
    if result.is_err() {
        panic!("can't append a span node to a buffer node")
    }
}

#[wasm_bindgen]
pub struct Emulator {
    cpu: Cpu,
    mem: Memory,
}

#[wasm_bindgen]
impl Emulator {
    pub fn new() -> Emulator {
        // Initialize for putting error messages to a browser console.
        utils::set_panic_hook();

        Emulator {
            cpu: Cpu::new(),
            mem: Memory::new(),
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset()
    }

    pub fn set_binary(&mut self, bin: Vec<u8>) {
        self.mem.set_binary(bin);
        log(&format!(
            "binary size: {} ({:#x})",
            self.mem.len(),
            self.mem.len()
        ));
    }

    pub fn execute(&mut self) {
        self.cpu.start(&mut self.mem);
        self.dump_registers();
    }

    pub fn dump_registers(&self) {
        for i in 0..REGISTERS_COUNT {
            let text = format!(
                "x{}: {:#x} ({}, {:#b})",
                i, self.cpu.xregs[i], self.cpu.xregs[i], self.cpu.xregs[i]
            );
            log(&text);
            render(&text);
        }

        let text = format!("---------------------");
        log(&text);
        render(&text);

        for i in 0..REGISTERS_COUNT {
            let text = format!("f{}: {:#?}", i, self.cpu.fregs[i]);
            log(&text);
            render(&text);
        }

        log(&format!("pc: {}", self.cpu.pc));
        render(&format!("pc: {}", self.cpu.pc));
    }
}
