mod cell;
mod data;
mod dialogs;
mod gridlayout;
mod monoglu_features;
mod tabs;

use dialogs::DialogStates;
use tabs::TabStates;

struct WebApp {
    is_logged_in: bool,
    dialog_states: DialogStates,
    tab_states: TabStates,
}

impl WebApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        monoglu_features::setup_fonts(&cc.egui_ctx);
        monoglu_features::configure_text_styles(&cc.egui_ctx);
        Self {
            is_logged_in: false,
            dialog_states: DialogStates::default(),
            tab_states: TabStates::new(),
        }
    }
}

impl eframe::App for WebApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.tab_states.update(ctx, frame);
        // self.dialog_states.update(ctx, frame);
    }
}

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
        Box::new(|cc| Box::new(WebApp::new(cc))),
    )
    .expect("failed to start eframe");
}