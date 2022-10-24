mod cell;
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

// -----------------------------------------------------------------------
// setup Engilsh / Korean fonts

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::empty();

    fonts.font_data.insert(
        "Roboto-Regular".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "..\\assets\\fonts\\Roboto-Regular.ttf"
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
                "NotoSansKR-Regular".to_owned(),
            ],
        );

    fonts
        .families
        .insert(
            egui::FontFamily::Monospace,
            vec![
                "Roboto-Regular".to_owned(),
                "NotoSansKR-Regular".to_owned(),
            ], 
        );

    ctx.set_fonts(fonts);
}

// ----------------------------------------------------------------
// setup verious text size

#[inline]
fn too_big() -> egui::TextStyle {
    egui::TextStyle::Name("TooBig".into())
}

#[inline]
fn heading3() -> egui::TextStyle {
    egui::TextStyle::Name("ContextHeading".into())
}

fn configure_text_styles(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (egui::TextStyle::Heading, egui::FontId::new(25.0, egui::FontFamily::Proportional)),
        (too_big(), egui::FontId::new(30.0, egui::FontFamily::Proportional)),
        (heading3(), egui::FontId::new(19.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Body, egui::FontId::new(16.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Monospace, egui::FontId::new(12.0, egui::FontFamily::Monospace)),
        (egui::TextStyle::Button, egui::FontId::new(12.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Small, egui::FontId::new(8.0, egui::FontFamily::Proportional)),
    ]
    .into();
    ctx.set_style(style);
}