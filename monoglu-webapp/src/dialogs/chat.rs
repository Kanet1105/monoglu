pub struct Chat {
    visible: bool,
    chat_inner: ChatInner,
}

impl Chat {
    pub fn new() -> Self {
        Self { 
            visible: false,
            chat_inner: ChatInner::default(),
        }
    }
}

impl super::Dialog for Chat {
    fn name(&self) -> &'static str {
        "Chatting"
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn toggle_visible(&mut self) {
        self.visible = !self.visible;
    }

    fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new(self.name())
            .open(&mut self.visible)
            .default_width(200.0)
            .resizable(true)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for (member, message) in &self.chat_inner.contents {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(format!("[{}]:", member));
                                    ui.label(format!("{}", message));
                                    ui.spacing_mut().item_spacing.y = 2.0;
                                });    
                            });
                        }
                        ui.scroll_to_cursor(Some(egui::Align::BOTTOM));        
                    });
    
                ui.separator();
                    
                egui::ScrollArea::vertical()
                    .id_source("second_scroll_area")
                    .max_height(40.0)
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        egui::TextEdit::multiline(&mut self.chat_inner.input_text)
                            .frame(false)
                            .hint_text("input here")
                            .show(ui);        
                    });
                    
                if &self.chat_inner.input_text.len() > &0 && ui.input_mut().key_pressed(egui::Key::Enter) {
                    self.chat_inner.contents.push(("You".to_owned(), self.chat_inner.input_text.clone().split("\n").collect()));
                    self.chat_inner.input_text = String::new();
                }
            });
    }
}

pub struct ChatInner {
    members: Vec<String>,
    contents: Vec<(String, String)>,
    input_text: String,
}

impl Default for ChatInner {
    fn default() -> Self {
        Self {
            members: vec!["You".to_owned(), "Box".to_owned()],
            contents: vec![("Bot".to_owned(), "Hello".to_owned())],
            input_text: String::new(),
        }
    }
}
