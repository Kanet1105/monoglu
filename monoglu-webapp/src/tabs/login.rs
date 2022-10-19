pub struct Login;

impl Login {
    pub fn new() -> Self {
        Self
    }
}

impl super::Tab for Login {
    fn name(&self) -> &'static str {
        "Login"
    }

    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
    }
}
