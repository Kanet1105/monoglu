mod info;
mod etc;
mod exception;
mod home;
mod login;
mod dashboard;
mod user;
use std::collections::HashMap;

use crate::gridlayout::{GridLayout, SizePolicy};
use crate::monoglu_features::Button;
pub trait Tab {
    fn name(&self) -> &'static str;
    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
}
pub struct TabStates {
    tabs: HashMap<String, Box<dyn Tab>>,
    top_layout: GridLayout,
}
impl TabStates {
    pub fn new() -> Self {
        let mut tabs = HashMap::<String, Box<dyn Tab>>::new();
        tabs.insert("#home".into(), Box::new(home::Home::new()));
        tabs.insert("#dashboard".into(), Box::new(dashboard::Dashboard::new()));
        tabs.insert("#info".into(), Box::new(info::Info::new()));
        tabs.insert("#etc".into(), Box::new(etc::Etc::new()));
        tabs.insert("#login".into(), Box::new(login::Login::new()));
        tabs.insert("#user".into(), Box::new(user::User::new()));

        Self {
            tabs,
            top_layout: top_layout()
        }
    }

    fn switch(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let current_tab = frame
        .info()
        .web_info
        .location
        .hash
        .to_owned();

        match self.tabs.get_mut(&current_tab) {
            Some(tab) => tab.view(&ctx, frame),
            None => exception::Exception::PageNotFound.view(&ctx, frame),
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("tab").min_height(30.0).show(ctx, |ui| {
            self.top_layout.show(ctx);
        });

        self.switch(ctx, frame);
    }
}

fn top_layout() -> GridLayout {
    let mut top_layout = GridLayout::new("top_layout".into(), 1, 1, SizePolicy::absolute(800.0, 40.0)); // panel size 에만 영향을 받아 의미가 없는듯??

    top_layout
        .get_grid(0, 0)
        .unwrap()
        .add_contents(Box::new(|ui: &mut egui::Ui| {
            ui.vertical(|ui| {
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.add_space(8.0);

                    ui.visuals_mut().button_frame = false;
                    let list = [
                        ("#home", "HOME"),
                        ("#dashboard", "DASHBOARD"),
                        ("#info", "INFO"),
                        ("#etc", "etc."),
                        ("#login", "SIGN IN"),
                        ("#user", "USER"),
                    ];
                    for (anchor, tab) in list {
                        if ui.add(Button::new(
                            egui::RichText::new(tab)
                                .text_style(egui::TextStyle::Body)
                                .color(egui::Color32::from_rgb(100, 100, 100))
                            )
                        ).clicked()
                        {
                            ui.output().open_url(anchor);
                        }
                    }
                });
                ui.add_space(8.0);
            });
        }));
    
    top_layout
}