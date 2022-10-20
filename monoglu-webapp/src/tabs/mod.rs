mod login;
mod user;

use std::collections::HashMap;

pub trait Tab {
    fn name(&self) -> &'static str;
    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
}

pub struct TabStates {
    tabs: HashMap<String, Box<dyn Tab>>,
    previous_tab: &'static str,
    current_tab: &'static str,
}

impl TabStates {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_switched(&self) -> bool {
        if self.current_tab == self.previous_tab {
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) -> bool {
        self.is_switched()
    }
}

impl Default for TabStates {
    fn default() -> Self {
        Self {
            tabs: HashMap::from([
                ("/#login".to_string(), Box::new(login::Login::new())),
                ("/#user".to_string(), Box::new(user::User::new())),
            ]),
            previous_tab: "/",
            current_tab: "/",
        }
    }
}
