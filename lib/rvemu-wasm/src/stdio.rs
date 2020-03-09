use rvemu_core::cpu::Cpu;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Input a message from the emulator console.
pub fn stdin(_cpu: Arc<Mutex<Cpu>>) {}

/// Output a message to the emulator console.
pub fn stdout(_cpu: Arc<Mutex<Cpu>>) {}

/// Output a message to the emulator console.
pub fn stdout_to_browser(message: &str) {
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

/// Output a message to both the browser console and the emulator console.
pub fn stdout_log(message: &str) {
    log(message);
    stdout_to_browser(message);
}
