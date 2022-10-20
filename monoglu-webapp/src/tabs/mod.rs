mod exception;
mod login;
mod user;

use exception::Exception;
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
    pub fn switch(&mut self, frame: &mut eframe::Frame) {
        self.current_tab = frame
            .info()
            .web_info
            .location
            .hash
            .to_owned();
    }

    pub fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.switch(frame);
        match self.tabs.get_mut(&self.current_tab) {
            Some(tab) => tab.view(ctx, frame),
            None => Exception::PageNotFound.view(ctx, frame),
        }
    }
}

impl Default for TabStates {
    fn default() -> Self {
        let mut tabs = HashMap::<String, Box<dyn Tab>>::new();
        tabs.insert("#login".to_string(), Box::new(login::Login::new()));
        tabs.insert("#user".to_string(), Box::new(user::User::new()));

        Self {
            tabs,
            current_tab: "/".to_string(),
        }
    }
}
