mod chat;
mod graph1;

pub trait Dialog {
    fn name(&self) -> &'static str;
    fn is_visible(&self) -> bool;
    fn toggle_visible(&mut self);
    fn show(&mut self, ctx: &egui::Context);
}

pub struct DialogStates {
    pub dialogs: Vec<Box<dyn Dialog>>,
}

impl DialogStates {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_bar")
            .min_width(ctx.available_rect().width() * 0.1)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Apps");
                ui.separator();

                for dialog in &mut self.dialogs {
                    let button = ui.button(dialog.name());
                    if button.clicked() {
                        dialog.toggle_visible();
                    }
                    if dialog.is_visible() {
                        dialog.show(ctx);
                    }
                }        
            });
        
    }
}

impl Default for DialogStates {
    fn default() -> Self {
        Self {
            dialogs: vec![
                Box::new(chat::Chat::new()),
                Box::new(graph1::Graph1::new()),
            ],
        }
    }
}
