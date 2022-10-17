mod apps;

struct WebApp;

impl WebApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self
    }

    fn navigation_bar(&self, ctx: &egui::Context, ratio: f32) {
        egui::TopBottomPanel::top("navigation_bar")
            .min_height(ctx.available_rect().height() * ratio)
            .resizable(false)
            .show(ctx, |ui| {});
    }

    fn side_bar(&self, ctx: &egui::Context, ratio: f32) {
        egui::SidePanel::left("side_bar")
            .min_width(ctx.available_rect().width() * ratio)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Apps");
                ui.separator();
            });
    }

    fn ui(&self, ctx: &egui::Context) {
        self.navigation_bar(ctx, 0.05);
        self.side_bar(ctx, 0.1);
    }
}

impl eframe::App for WebApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ui(ctx);
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
