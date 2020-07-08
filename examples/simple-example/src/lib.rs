#[macro_use]
extern crate log;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

fn init_log() {
    use log::Level;
    console_log::init_with_level(Level::Trace).expect("Error initializing log");
}

pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(start)]
pub fn main() {
    init_log();
    set_panic_hook();

    info!("{}", web_butter::add(1, 2));
}
