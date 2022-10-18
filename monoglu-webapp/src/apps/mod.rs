mod chat;
mod login;

pub use chat::Chat;
pub use login::Login;

pub trait Application {
    fn view(&mut self, ctx: &egui::Context);
}

pub enum Size {
    Responsive,
    ResponsiveRange,
}

impl Size {
    pub fn responsive() -> Self {
        Self::Responsive
    }

    pub fn ResponsiveRange() -> Self {
        Self::ResponsiveRange
    }
}
