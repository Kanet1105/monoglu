pub trait Dialog {
    fn name(&self) -> &'static str;
    fn is_visible(&self) -> bool;
    fn show(&self, ctx: &egui::Context);
}

pub struct DialogStates {
    dialog_list: Vec<Box<dyn Dialog>>,
}

impl DialogStates {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for dialog in &self.dialog_list {
            if dialog.is_visible() {
                dialog.show(ctx);
            }
        }
    }
}

impl Default for DialogStates {
    fn default() -> Self {
        Self {
            dialog_list: vec![],
        }
    }
}
