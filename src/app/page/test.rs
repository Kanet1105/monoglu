use crate::app::component::NavigationBar;

pub struct Test;

impl Test {
    pub fn view(ctx: &egui::Context) {
        egui::TopBottomPanel::top("navigator").show(ctx, |ui| {
            ui.add(NavigationBar::new());
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Click me").clicked() {
                // take some action here
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Button::new("Button_1"));
            ui.horizontal(|ui| {
                ui.add(egui::Button::new("button_2"));
                ui.add(egui::Button::new("button_3"));
            });
            ui.add(egui::Button::new("button_4"));
        });
    }
}