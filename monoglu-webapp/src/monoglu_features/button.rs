/// Monoglu button widget
pub struct Button {
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

impl Button {
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

impl egui::Widget for Button {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Button {
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
        desired_size = desired_size.max(min_size);

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
