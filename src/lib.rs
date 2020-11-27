#![recursion_limit = "256"]

use types::Model;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod initial;
mod types;
mod view;

#[wasm_bindgen(start)]
pub fn main() {
    web_logger::init();
    log::info!("starting");
    // See https://github.com/rustwasm/console_error_panic_hook/issues/8.
    // std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    App::<Model>::new().mount_to_body();
}
