#![recursion_limit = "512"]

use types::Model;
use wasm_bindgen::prelude::*;

mod initial;
mod schema;
mod types;
mod view;

#[wasm_bindgen(start)]
pub fn main() {
    // web_logger::init();
    // wasm_logger::init(wasm_logger::Config::default().module_prefix("some::module"));
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));

    log::info!("starting");
    // See https://github.com/rustwasm/console_error_panic_hook/issues/8.
    // std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    // App::<Model>::new().mount_to_body();
    yew::start_app::<Model>();
}
