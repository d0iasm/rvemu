#![feature(generators, generator_trait)]

mod utils;

use std::cell::RefCell;
use std::ops::Generator;
use std::pin::Pin;
use std::rc::Rc;

use rvemu_core::bus::DRAM_BASE;
use rvemu_core::emulator;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

/*
/// Wrapper for rvemu::emulator::Emulator to connect to WebAssembly.
#[wasm_bindgen]
pub struct Emulator {
    emu: Rc<RefCell<emulator::Emulator>>,
}
*/

/// Output a message to the emulator console.
pub fn stdout(message: &str) {
    let document = window()
        .document()
        .expect("should have a document on window");
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

/// Output a message to both the browser console and the emulator console.
pub fn stdout_log(message: &str) {
    log(message);
    //stdout(message);
    //for c in message.chars() {
    //   write_to_buffer(c as u8);
    //}
}

#[wasm_bindgen]
pub fn emulator_start(kernel: Vec<u8>, fsimg: Vec<u8>) {
    utils::set_panic_hook();

    let mut emu = emulator::Emulator::new();
    emu.set_dram(kernel);
    emu.set_disk(fsimg);
    emu.set_pc(DRAM_BASE);

    let mut count = 0;
    let key_event = Closure::wrap(Box::new(move || {
        log(&format!("count: {}", count));
        count += 1;
    }) as Box<dyn FnMut()>);
    let document = window()
        .document()
        .expect("should have a document on window");
    //document.set_onkeypress(Some(key_event.as_ref().unchecked_ref()));
    document.set_onclick(Some(key_event.as_ref().unchecked_ref()));
    key_event.forget();

    let mut generator = move || {
        let mut count = 0;
        loop {
            count += 1;
            // 1. Fetch.
            let data_or_error = emu.cpu.fetch();

            // 2. Add 4 to the program counter.
            emu.cpu.pc += 4;
            emu.cpu.timer_increment();

            // 3. Decode.
            // 4. Execution.
            let _result = match data_or_error {
                Ok(data) => match emu.cpu.execute(data) {
                    Ok(_) => Ok(()),
                    Err(exception) => exception.take_trap(&mut emu.cpu),
                },
                Err(exception) => exception.take_trap(&mut emu.cpu),
            };

            // Take an interrupt.
            match emu.cpu.check_interrupt() {
                Some(interrupt) => interrupt.take_trap(&mut emu.cpu),
                None => {}
            }

            if count > 100000 {
                //log(&format!("count in generator: {}", count));
                count = 0;
                yield;
            }
        }
    };

    let animation_handler = Rc::new(RefCell::new(None));
    let cloned_animation_handler = animation_handler.clone();

    *cloned_animation_handler.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        Pin::new(&mut generator).resume();
        request_animation_frame(animation_handler.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(cloned_animation_handler.borrow().as_ref().unwrap());
}

/*
#[wasm_bindgen]
impl Emulator {
    /// Constructor for the emulator.
    pub fn new() -> Emulator {
        // Initialize for putting error messages to a browser console.
        utils::set_panic_hook();

        Self {
            emu: Rc::new(RefCell::new(emulator::Emulator::new())),
        }
    }

    /// Reset the emulator.
    pub fn reset(&mut self) {
        (*self.emu.borrow_mut()).reset();
    }

    /// Set binary data to the beginning of the DRAM from the emulator console of a browser.
    pub fn set_dram(&mut self, data: Vec<u8>) {
        (*self.emu.borrow_mut()).set_dram(data);
    }

    /// Set binary data to the virtio disk from the emulator console.
    pub fn set_disk(&mut self, data: Vec<u8>) {
        (*self.emu.borrow_mut()).set_disk(data);
    }

    /// Start executing.
    pub fn start(&mut self) {
        /*
            (*self.emu.borrow_mut()).set_pc(DRAM_BASE);

            let mut count = 0;
            let key_event = Closure::wrap(Box::new(move || {
                log(&format!("count: {}", count));
                count += 1;
            }) as Box<dyn FnMut()>);
            window().set_onkeydown(Some(key_event.as_ref().unchecked_ref()));
            key_event.forget();

            let mut cloned_emu = self.emu.clone();
            let mut generator = move || {
                let window = web_sys::window().expect("no global `window` exists");
                let document = window.document().expect("should have a document on window");
                //let input = document.get_element_by_id("input").unwrap();
                let mut count = 0;
                loop {
                    let emu = *cloned_emu.borrow_mut();
                    count += 1;
                    // 1. Fetch.
                    let data_or_error = emu.cpu.fetch();

                    // 2. Add 4 to the program counter.
                    emu.cpu.pc += 4;
                    emu.cpu.timer_increment();

                    // 3. Decode.
                    // 4. Execution.
                    let _result = match data_or_error {
                        Ok(data) => match emu.cpu.execute(data) {
                            Ok(_) => Ok(()),
                            Err(exception) => exception.take_trap(&mut emu.cpu),
                        },
                        Err(exception) => exception.take_trap(&mut emu.cpu),
                    };

                    // Take an interrupt.
                    match emu.cpu.check_interrupt() {
                        Some(interrupt) => interrupt.take_trap(&mut emu.cpu),
                        None => {}
                    }

                    /*
                    if count > 10000 {
                        yield;
                    }
                    */
                    log(&format!("count in generator: {}", count));
                    //input.set_text_content(Some(&format!("{}", count).to_string()));
                }
            };
        */

        /*
        let animation_handler = Rc::new(RefCell::new(None));
        let cloned_animation_handler = animation_handler.clone();

        *cloned_animation_handler.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            Pin::new(&mut generator).resume();
            request_animation_frame(animation_handler.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(cloned_animation_handler.borrow().as_ref().unwrap());
        */
    }

    /// Output current registers.
    pub fn dump_registers(&self) {
        let emu = self.emu.borrow();
        stdout_log(&format!("{}", emu.cpu.xregs));
        stdout_log(&format!(
            "-------------------------------------------------------------------------------------------"
        ));
        stdout_log(&format!("{}", emu.cpu.fregs));
        stdout_log(&format!(
            "-------------------------------------------------------------------------------------------"
        ));
        stdout_log(&format!("{}", emu.cpu.state));
        stdout_log(&format!(
            "-------------------------------------------------------------------------------------------"
        ));
        stdout_log(&format!("pc: {}", emu.cpu.pc));
    }
}
*/
