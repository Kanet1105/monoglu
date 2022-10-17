mod apps;

#[cfg(target_arch = "wasm32")]
pub fn run() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    // Enable logging on the browser console.
    wasm_logger::init(wasm_logger::Config::default());

    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "main_canvas", // hardcode it
        web_options,
        Box::new(|cc| Box::new(apps::WebApp::new(cc))),
    )
    .expect("failed to start eframe");
}
