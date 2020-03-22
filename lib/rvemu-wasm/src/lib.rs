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

/// Get a global window object.
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

/// Sets a timer which executes a function or specified piece of code once the timer expires.
fn set_timeout_with_callback(f: &Closure<dyn FnMut()>, timeout: i32) {
    window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(f.as_ref().unchecked_ref(), timeout)
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen]
pub fn emulator_start(kernel: Vec<u8>, fsimg: Vec<u8>) {
    utils::set_panic_hook();

    let mut emu = emulator::Emulator::new();
    emu.set_dram(kernel);
    emu.set_disk(fsimg);
    emu.set_pc(DRAM_BASE);

    let mut generator = move || {
        let mut count: u64 = 0;
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

            if count > 500000 {
                count = 0;
                yield;
            }
        }
    };

    let handler = Rc::new(RefCell::new(None));
    let cloned_handler = handler.clone();

    // Set a timer to execute the emulator again.
    *cloned_handler.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        Pin::new(&mut generator).resume(());
        set_timeout_with_callback(handler.borrow().as_ref().unwrap(), 0);
    }) as Box<dyn FnMut()>));

    set_timeout_with_callback(cloned_handler.borrow().as_ref().unwrap(), 0);
}
