// mod components;
mod dialogs;
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
        setup_fonts(&cc.egui_ctx);
        Self {
            is_logged_in: false,
            dialog_states: DialogStates::default(),
            tab_states: TabStates::default(),
        }
    }
}

impl eframe::App for WebApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.dialog_states.update(ctx, frame);
        self.tab_states.update(ctx, frame);
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


fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::empty();

    fonts.font_data.insert(
        "Roboto-Regular".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "..\\assets\\fonts\\Roboto-Regular.ttf"
        )),
    );

    fonts.font_data.insert(
        "Roboto-Italic".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "..\\assets\\fonts\\Roboto-Italic.ttf"
        )),
    );

    fonts.font_data.insert(
        "Roboto-Thin".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "..\\assets\\fonts\\Roboto-Thin.ttf"
        )),
    );

    fonts.font_data.insert(
        "Roboto-ThinItalic".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "..\\assets\\fonts\\Roboto-ThinItalic.ttf"
        )),
    );

    fonts.font_data.insert(
        "Roboto-Bold".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "..\\assets\\fonts\\Roboto-Bold.ttf"
        )),
    );

    fonts.font_data.insert(
        "Roboto-BoldItalic".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "..\\assets\\fonts\\Roboto-BoldItalic.ttf"
        )),
    );

    fonts.font_data.insert(
        "NotoSansKR-Regular".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "..\\assets\\fonts\\NotoSansKR-Regular.otf"
        )),
    );

    fonts
        .families
        .insert(
            egui::FontFamily::Proportional,
            vec![
                "Roboto-Regular".to_owned(),
                "Roboto-Italic".to_owned(),
                "Roboto-Thin".to_owned(),
                "Roboto-ThinItalic".to_owned(),
                "Roboto-Bold".to_owned(),
                "Roboto-BoldItalic".to_owned(),
                "NotoSansKR-Regular".to_owned(),
            ],
        );

    fonts
        .families
        .insert(
            egui::FontFamily::Monospace,
            vec![
                "Roboto-Regular".to_owned(),
                "Roboto-Italic".to_owned(),
                "Roboto-Thin".to_owned(),
                "Roboto-ThinItalic".to_owned(),
                "Roboto-Bold".to_owned(),
                "Roboto-BoldItalic".to_owned(),
                "NotoSansKR-Regular".to_owned(),
            ], 
        );

    ctx.set_fonts(fonts);
}