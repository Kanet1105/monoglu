mod info;
mod etc;
mod exception;
mod home;
mod login;
mod dashboard;
mod user;

use std::collections::HashMap;

pub trait Tab {
    fn name(&self) -> &'static str;
    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
}

pub struct TabStates {
    tabs: HashMap<String, Box<dyn Tab>>,
    current_tab: String,
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
            current_tab: "/".to_string(), 
        }
    }

    fn main_view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("tab").show(&ctx, |ui| {
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

                if self.current_tab == "/".to_string() {
                    self.current_tab = "#home".to_string()
                }

                let mut selected_anchor = self.current_tab.clone();
                for (anchor, tab) in list {
                    if ui.add(
                        crate::monoglu_features::SelectableLabel::new(
                            &selected_anchor == anchor,
                            tab
                        )
                    ).clicked()
                    {
                        selected_anchor = anchor.into();
                        if frame.is_web() {
                            ui.output().open_url(anchor.to_string());
                        }
                    }
                }
                self.current_tab = selected_anchor;

            });
        });

        self.switch(frame);
        match self.tabs.get_mut(&self.current_tab) {
            Some(tab) => tab.view(ctx, frame),
            None => exception::Exception::PageNotFound.view(ctx, frame),
        }
    }

    pub fn switch(&mut self, frame: &mut eframe::Frame) {
        self.current_tab = frame
            .info()
            .web_info
            .location
            .hash
            .to_owned();
    }

    pub fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {


        egui::CentralPanel::default().show(ctx, |ui| {
            self.main_view(ctx, frame);
        });    


    }

}
    
