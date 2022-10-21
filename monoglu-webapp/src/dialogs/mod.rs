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
        egui::SidePanel::left("side_bar")
            .min_width(ctx.available_rect().width() * 0.1)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Apps");
                ui.separator();

                for dialog in &mut self.dialogs {
                    let button = ui.add(
                        SideButton::new(dialog.name())
                    
                    );
                    if button.clicked() {
                        dialog.toggle_visible();
                    }
                    if dialog.is_visible() {
                        dialog.show(ctx);
                    }
                }
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
}

impl SideButton {
    pub fn new(text: impl Into<egui::WidgetText>) -> Self {
        Self {
            text: text.into(),
            wrap: None,
            fill: Some(egui::Color32::WHITE),
            stroke: Some(egui::Stroke { width: 1.0, color: egui::Color32::BLUE }),
            sense: egui::Sense::click(),
            small: false,
            frame: None,
            min_size: egui::Vec2::ZERO,
            image: None,
        }
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
        } = self;

        let frame = frame.unwrap_or_else(|| ui.visuals().button_frame);

        let mut button_padding = ui.spacing().button_padding + egui::vec2(30.0, 30.0);
        
        let total_extra = button_padding + button_padding;

        let wrap_width = ui.available_width() - total_extra.x;
        let text = text.into_galley(ui, wrap, wrap_width, egui::TextStyle::Button);

        let mut desired_size = text.size() + 2.0 * button_padding;
        desired_size = desired_size.at_least(min_size);

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

        if let Some(image) = image {
            let image_rect = egui::Rect::from_min_size(
                egui::pos2(rect.min.x, rect.center().y -0.5 - (image.size().y / 2.0)),
                image.size(),
            );
            image.paint_at(ui, image_rect);
        }

        response
    }
}