fn main() {
    // See https://github.com/rustwasm/console_error_panic_hook/issues/8.
    // std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    yew::start_app::<linc::Model>();
}
