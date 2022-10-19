mod ex1;
mod ex2;

pub trait Dialog {
    fn name(&self) -> &'static str;
    fn is_visible(&self) -> bool;
    fn show(&mut self, ctx: &egui::Context);
}

pub struct DialogStates {
    dialog_list: Vec<Box<dyn Dialog>>,
}

impl DialogStates {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for dialog in &mut self.dialog_list {
            if dialog.is_visible() {
                dialog.show(ctx);
            }
        }
    }
}

impl Default for DialogStates {
    fn default() -> Self {
        Self {
            dialog_list: vec![
                Box::new(ex1::Ex1::new()),
                Box::new(ex2::Ex2::new()),
            ],
        }
    }
}
