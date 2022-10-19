mod dialogs;
mod tabs;

use dialogs::DialogStates;

struct WebApp {
    dialogs: DialogStates,
}

impl WebApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            dialogs: DialogStates::new(),
        }
    }

    // fn navigation_bar(&self, ctx: &egui::Context, ratio: f32) {
    //     let frame = egui::Frame::none()
    //         .inner_margin(Margin { left: 15.0, right: 15.0, top: 15.0, bottom: 15.0 })
    //         .fill(Color32::GRAY);
    //     egui::TopBottomPanel::top("navigation_bar")
    //         .frame(frame)
    //         .min_height(ctx.available_rect().height() * ratio)
    //         .resizable(false)
    //         .show(ctx, |ui| {
    //             ui.horizontal(|ui| {
    //                 ui.heading("Monoglu");
    //             });
    //         });
    // }

    fn side_bar(&mut self, ctx: &egui::Context, ratio: f32) {
        egui::SidePanel::left("side_bar")
            .min_width(ctx.available_rect().width() * ratio)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Apps");
                ui.separator();

                for dialog in &mut self.dialogs.dialog_list {
                    let button = ui.button(dialog.name());
                    if button.clicked() {
                        dialog.set_visible(true);
                    }
                }
            });
    }
}

impl eframe::App for WebApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.side_bar(ctx, 0.1);
        self.dialogs.update(ctx, frame);
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
