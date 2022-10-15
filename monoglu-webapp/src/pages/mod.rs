mod login;
pub use login::LoginPage;

mod user;
pub use user::UserPage;

pub trait View {
    fn show(ctx: &egui::Context);
}
