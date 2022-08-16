use monoglu::prelude::*;

#[cfg(target_arch = "wasm32")]
fn main() {
    // init logger
    wasm_logger::init(wasm_logger::Config::default());

    // an egui basic setup.
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    eframe::start_web(
        "main_canvas",
        Box::new(|cc| Box::new(Application::new(cc))),
    )
    .expect("failed to load eframe.");
}