#![feature(generators, generator_trait)]

mod utils;

use std::cell::RefCell;
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;
use std::rc::Rc;

use rvemu_core::bus::DRAM_BASE;
use rvemu_core::cpu::Cpu;
use rvemu_core::emulator;
use rvemu_core::exception::Trap;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Get a global window object.
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

/// Sets a timer which executes a function or specified piece of code once the timer expires.
fn set_timeout_with_callback(f: &Closure<dyn FnMut()>, timeout: i32) {
    window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(f.as_ref().unchecked_ref(), timeout)
        .expect("failed to regsiter setTimeout");
}

/// Set an active text to the flag element, which represents the emulator is executing.
fn set_executing() {
    let document = window().document().expect("failed to get a document");
    let flag = document
        .get_element_by_id("flag")
        .expect("failed to get the `flag` element");
    flag.set_text_content(Some("active"));
}

/// Set a inactive text to the flag element, which represents the emulator is finished.
fn clear_executing() {
    let document = window().document().expect("failed to get a document");
    let flag = document
        .get_element_by_id("flag")
        .expect("failed to get the `flag` element");
    flag.set_text_content(Some("inactive"));
}

/// Add a new element to the output buffer in JS.
fn output(message: &str) {
    let document = window().document().expect("failed to get a document");
    let buffer = document
        .get_element_by_id("outputBuffer")
        .expect("failed to get the `outputBuffer` element");

    let span = document
        .create_element("span")
        .expect("failed to create a new span element");
    span.set_inner_html(message);
    let result = buffer.append_child(&span);
    if result.is_err() {
        panic!("can't append a span node to a buffer node")
    }
}

/// Output current registers to UART.
fn dump_registers(cpu: &mut Cpu) {
    output(&format!("-------------------------------------------------------------------------------------------"));
    output(&format!("{}", cpu.xregs));
    output(&format!("-------------------------------------------------------------------------------------------"));
    output(&format!("{}", cpu.fregs));
    output(&format!("-------------------------------------------------------------------------------------------"));
    output(&format!("{}", cpu.state));
    output(&format!("-------------------------------------------------------------------------------------------"));
    output(&format!("pc: {:#x}", cpu.pc));

    log(&format!("-------------------------------------------------------------------------------------------"));
    log(&format!("{}", cpu.xregs));
    log(&format!("-------------------------------------------------------------------------------------------"));
    log(&format!("{}", cpu.fregs));
    log(&format!("-------------------------------------------------------------------------------------------"));
    log(&format!("{}", cpu.state));
    log(&format!("-------------------------------------------------------------------------------------------"));
    log(&format!("pc: {:#x}", cpu.pc));
}

#[wasm_bindgen]
pub fn emulator_start(kernel: Vec<u8>, fsimg: Option<Vec<u8>>) {
    utils::set_panic_hook();

    let mut emu = emulator::Emulator::new();
    emu.initialize_dram(kernel);
    if let Some(fsimg) = fsimg {
        emu.initialize_disk(fsimg);
    }
    emu.initialize_pc(DRAM_BASE);

    set_executing();

    let mut generator = move || {
        let mut count: u64 = 0;
        loop {
            count += 1;
            if count > 500000 {
                count = 0;
                yield;
            }

            // Take an interrupt.
            match emu.cpu.check_pending_interrupt() {
                Some(interrupt) => interrupt.take_trap(&mut emu.cpu),
                None => {}
            }

            // Increment a CPU timer for a timer interrupt.
            emu.cpu.timer_increment();

            // Increment a CPU clock. In one cycle, CPU does fetch, decode, and execute.
            let trap = match emu.cpu.tick() {
                Ok(_) => Trap::Requested, // dummy
                Err(exception) => exception.take_trap(&mut emu.cpu),
            };

            match trap {
                Trap::Requested => {}
                _ => {
                    // End of the program.
                    return emu;
                }
            }
        }
    };

    let handler = Rc::new(RefCell::new(None));
    let cloned_handler = handler.clone();

    // Set a timer to execute the emulator again.
    *cloned_handler.borrow_mut() =
        Some(Closure::wrap(
            Box::new(move || match Pin::new(&mut generator).resume(()) {
                GeneratorState::Complete(mut emu) => {
                    clear_executing();
                    dump_registers(&mut emu.cpu);
                }
                _ => {
                    set_timeout_with_callback(handler.borrow().as_ref().unwrap(), 0);
                }
            }) as Box<dyn FnMut()>,
        ));

    set_timeout_with_callback(cloned_handler.borrow().as_ref().unwrap(), 0);
}
