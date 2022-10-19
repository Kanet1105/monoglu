mod login;
mod user;

pub trait Tab {
    fn view(&mut self, ctx: &egui::Context);
}

pub struct TabStates {}
