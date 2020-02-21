pub mod cpu;
pub mod csr;
//pub mod elf;
pub mod exception;
pub mod memory;
mod utils;

use crate::cpu::*;
use crate::csr::*;
//use crate::elf::*;
use crate::exception::*;
use crate::memory::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Output a message to an emulator console.
fn render(message: &str) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let buffer = document
        .get_element_by_id("buffer")
        .expect("should have a element with a `buffer` id");

    let span = document
        .create_element("span")
        .expect("span element should be created successfully");
    span.set_inner_html(message);
    let result = buffer.append_child(&span);
    if result.is_err() {
        panic!("can't append a span node to a buffer node")
    }
}

/// Output a message to both a browser console and an emulator console.
pub fn output(message: &str) {
    log(message);
    render(message);
}

/// An emulator struct to holds a CPU and a memory.
#[wasm_bindgen]
pub struct Emulator {
    cpu: Cpu,
    mem: Memory,
}

#[wasm_bindgen]
impl Emulator {
    /// Constructor for an emulator.
    pub fn new() -> Emulator {
        // Initialize for putting error messages to a browser console.
        utils::set_panic_hook();

        Emulator {
            cpu: Cpu::new(),
            mem: Memory::new(),
        }
    }

    /// Reset CPU state.
    pub fn reset(&mut self) {
        self.cpu.reset()
    }

    /// Set a binary from the emulator console of a browser.
    pub fn set_binary(&mut self, bin: Vec<u8>) {
        //let header = Elf64Ehdr::new(&bin);
        //if !header.verify() {
        //output(&format!("unexpected ELF format"))
        //}
        //log(&format!("{:#?}", header));
        // Set an entry point. Divide 8 because `e_entry` is the number of bits.
        //self.cpu.pc = header.e_entry as usize;

        self.mem.set_binary(bin);
    }

    /// Start executing.
    pub fn execute(&mut self) {
        self.cpu.start(&mut self.mem);
        self.dump_registers();
    }

    /// Output current registers.
    pub fn dump_registers(&self) {
        for i in 0..REGISTERS_COUNT {
            output(&format!(
                "x{}: {:#x} ({}, {:#b})",
                i, self.cpu.xregs[i], self.cpu.xregs[i], self.cpu.xregs[i]
            ));
        }

        output(&format!("---------------------"));

        for i in 0..REGISTERS_COUNT {
            output(&format!("f{}: {:#?}", i, self.cpu.fregs[i]));
        }

        output(&format!("pc: {}", self.cpu.pc));
    }
}
