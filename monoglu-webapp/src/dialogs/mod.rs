use egui::NumExt;

mod chat;
mod graph1;

pub trait Dialog {
    fn name(&self) -> &'static str;
    fn is_visible(&self) -> bool;
    fn toggle_visible(&mut self);
    fn show(&mut self, ctx: &egui::Context);
}

pub struct DialogStates {
    pub dialogs: Vec<Box<dyn Dialog>>,
}

impl DialogStates {
    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let min_width = ctx.available_rect().width() * 0.1;

        egui::SidePanel::left("side_bar")
            .min_width(min_width)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Apps");
                ui.separator();

                for dialog in &mut self.dialogs {
        
                    let button = ui.add(
                        SideButton::new(dialog.name())
                            .custom_size(egui::vec2(ui.available_width(), 50.0))
                            .left_offset(10.0)
                            .fill(egui::Color32::YELLOW)
                            .stroke(1.0, egui::Color32::GREEN)
                    );
                    if button.clicked() {
                        dialog.toggle_visible();
                    }
                    if dialog.is_visible() {
                        dialog.show(ctx);
                    }
                }

                ui.add(
                    SideButton::new("안녕하세요")
                        .custom_size(egui::vec2(ui.available_width(), 30.0))
                        .vertical_centered_text(true)
                        .remove_stroke()                
                );
                ui.add(
                    SideButton::new(egui::RichText::new("모노글루입니다.").color(egui::Color32::RED).italics())
                        .custom_size(egui::vec2(ui.available_width(), 30.0))
                        .vertical_centered_text(true)
                        .fill(egui::Color32::LIGHT_BLUE)
                        .stroke(2.0, egui::Color32::BLACK)                        
                );
                ui.add(
                    SideButton::new(egui::RichText::new("모노글루입니다.").text_style(egui::TextStyle::Heading).strong())
                        .custom_size(egui::vec2(ui.available_width(), 30.0))
                        .vertical_centered_text(true)
                        .fill(egui::Color32::LIGHT_BLUE)
                        .stroke(2.0, egui::Color32::BLACK)                        
                );
                ui.add(
                    SideButton::new(egui::RichText::new("모노글루입니다.").text_style(egui::TextStyle::Small).weak())
                        .custom_size(egui::vec2(ui.available_width(), 30.0))
                        .vertical_centered_text(true)
                        .fill(egui::Color32::LIGHT_BLUE)
                        .stroke(2.0, egui::Color32::BLACK)                        
                );

            });
    }
}

impl Default for DialogStates {
    fn default() -> Self {
        Self {
            dialogs: vec![
                Box::new(chat::Chat::new()),
                Box::new(graph1::Graph1::new()),
            ],
        }
    }
}

/// Custom widget button for side bar.
pub struct SideButton {
    text: egui::WidgetText,
    wrap: Option<bool>,
    fill: Option<egui::Color32>,
    stroke: Option<egui::Stroke>,
    sense: egui::Sense,
    small: bool,
    frame: Option<bool>,
    min_size: egui::Vec2,
    image: Option<egui::widgets::Image>,
    custom_size: Option<egui::Vec2>,
    left_offset: f32,
    vertical_centered_text: bool,
}

impl SideButton {
    pub fn new(text: impl Into<egui::WidgetText>) -> Self {
        Self {
            text: text.into(),
            wrap: None,
            fill: Some(egui::Color32::WHITE),
            stroke: Some(egui::Stroke { width: 1.0, color: egui::Color32::GRAY }),
            sense: egui::Sense::click(),
            small: false,
            frame: Some(true),
            min_size: egui::Vec2::ZERO,
            image: None,
            custom_size: None,
            left_offset: 0.0,
            vertical_centered_text: false,
        }
    }

    pub fn fill(mut self, fill: impl Into<egui::Color32>) -> Self {
        self.fill = Some(fill.into());
        self.frame = Some(true);
        self
    }

    pub fn stroke(mut self, width: f32, color: egui::Color32) -> Self {
        self.stroke = Some(egui::Stroke { width: width, color: color });
        self.frame = Some(true);
        self
    }

    pub fn remove_stroke(mut self) -> Self {
        self.stroke = None;
        self.frame = Some(true);
        self
    } 

    pub fn custom_size(mut self, size: egui::Vec2) -> Self {
        self.custom_size = Some(size);
        self
    }

    pub fn left_offset(mut self, left_offset: f32) -> Self {
        self.left_offset = left_offset;
        self
    }

    pub fn vertical_centered_text(mut self, vertical_centered_text: bool) -> Self {
        self.vertical_centered_text = vertical_centered_text;
        self
    }

}

impl egui::Widget for SideButton {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let SideButton {
            text,
            wrap,
            fill,
            stroke,
            sense,
            small,
            frame,
            min_size,
            image,
            custom_size,
            left_offset,
            vertical_centered_text,
        } = self;

        let frame = frame.unwrap_or_else(|| ui.visuals().button_frame);

        let mut button_padding = ui.spacing().button_padding + egui::vec2(left_offset, 0.0);
        
        let total_extra = button_padding + button_padding;

        let wrap_width = ui.available_width() - total_extra.x;
        let text = text.into_galley(ui, wrap, wrap_width, egui::TextStyle::Button);

        let mut desired_size: egui::Vec2;
        match custom_size {
            Some(custom_size) => desired_size = custom_size,
            None => desired_size = text.size() + 2.0 * button_padding,
        }
        desired_size = desired_size.at_least(min_size);

        if vertical_centered_text {
            button_padding = (desired_size - text.size()) / 2.0;
        }

        let (rect, response) = ui.allocate_at_least(desired_size, sense);
        response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, text.text()));

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);
            let text_pos = if let Some(image) = image {
                let icon_spacing = ui.spacing().icon_spacing;
                egui::emath::pos2 (
                    rect.min.x + button_padding.x + image.size().x + icon_spacing,
                    rect.center().y - 0.5 * text.size().y,
                )
            } else {
                ui.layout()
                    .align_size_within_rect(text.size(), rect.shrink2(button_padding))
                    .min
            };

            if frame {
                let fill = fill.unwrap_or(visuals.bg_fill);
                let stroke = stroke.unwrap_or(visuals.bg_stroke);
                ui.painter().rect(
                    rect.expand(visuals.expansion),
                    visuals.rounding,
                    fill,
                    stroke,
                );
            }

            text.paint_with_visuals(ui.painter(), text_pos, visuals);
        }

        response
    }
}
