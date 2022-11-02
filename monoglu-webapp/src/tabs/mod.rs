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
    grid_layout: GridLayout,
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

        let mut grid_layout = GridLayout::new("top_panel".into(), 1, 1, SizePolicy::absolute(20.0, 500.0));

        grid_layout
            .get_grid(0, 0)
            .unwrap()
            .add_contents(Box::new(|ui: &mut egui::Ui| {
                ui.horizontal(|ui| {
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
                        if ui.add(Button::new(tab)).clicked()
                        {
                            ui.output().open_url(anchor);
                        }
                    }
                });
            }));
      
        Self {
            tabs,
            grid_layout,
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
        egui::TopBottomPanel::top("tab").show(ctx, |ui| {
            self.grid_layout.show(ctx);
        });

        self.switch(ctx, frame);
    }
}

