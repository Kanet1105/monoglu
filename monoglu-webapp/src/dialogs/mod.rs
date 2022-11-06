mod chat;
mod graph1;

use super::monoglu_features::Button;


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
    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let min_width = ctx.available_rect().width() * 0.1;

        egui::SidePanel::left("side_bar")
            .min_width(min_width)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Apps");
                ui.separator();

                for dialog in &mut self.dialogs {
        
                    let button = ui.add(
                        Button::new(dialog.name())
                            .custom_size(egui::vec2(ui.available_width(), 50.0))
                            .left_offset(10.0)
                            .fill(egui::Color32::YELLOW)
                            .stroke(1.0, egui::Color32::GREEN)
                    );
                    if button.clicked() {
                        dialog.toggle_visible();
                    }
                    if dialog.is_visible() {
                        dialog.show(ctx);
                    }
                }

                ui.add(
                    Button::new("안녕하세요")
                        .custom_size(egui::vec2(ui.available_width(), 30.0))
                        .vertical_centered_text(true)
                        .remove_stroke()                
                );
                ui.add(
                    Button::new(egui::RichText::new("모노글루입니다.").color(egui::Color32::RED).italics())
                        .custom_size(egui::vec2(ui.available_width(), 30.0))
                        .vertical_centered_text(true)
                        .fill(egui::Color32::LIGHT_BLUE)
                        .stroke(2.0, egui::Color32::BLACK)                        
                );
                ui.add(
                    Button::new(egui::RichText::new("모노글루입니다.").text_style(egui::TextStyle::Small).weak())
                        .custom_size(egui::vec2(ui.available_width(), 30.0))
                        .vertical_centered_text(true)
                        .fill(egui::Color32::LIGHT_BLUE)
                        .stroke(2.0, egui::Color32::BLACK)                        
                );
                ui.add(
                    Button::new(egui::RichText::new("모노글루입니다.").text_style(egui::TextStyle::Name("TooBig".into())).strong())
                        .custom_size(egui::vec2(ui.available_width(), 30.0))
                        .vertical_centered_text(true)
                        .fill(egui::Color32::LIGHT_BLUE)
                        .stroke(2.0, egui::Color32::BLACK)                        
                );

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

