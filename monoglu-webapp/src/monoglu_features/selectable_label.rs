/// Monoglu Selectable Label Widget
pub struct SelectableLabel {
    selected: bool,
    text: egui::WidgetText,
    custom_size: Option<egui::Vec2>,
    fill: egui::Color32,
    rounding: egui::epaint::Rounding,
    stroke: egui::Stroke,
}

impl SelectableLabel {
    pub fn new(selected: bool, text: impl Into<egui::WidgetText>) -> Self {
        Self {
            selected,
            text: text.into(),
            custom_size: None,
            fill: egui::Color32::GRAY,
            rounding: egui::epaint::Rounding::same(3.0),
            stroke: egui::Stroke::default(),
        }
    }

    pub fn fill(mut self, fill: egui::Color32) -> Self {
        self.fill = fill;
        self
    }

    pub fn rounding(mut self, rounding: egui::epaint::Rounding) -> Self {
        self.rounding = rounding;
        self
    }

    pub fn stroke(mut self, width: f32, color: egui::Color32) -> Self {
        self.stroke = egui::Stroke {width: width, color: color};
        self
    }

    pub fn custom_size(mut self, size: egui::Vec2) -> Self {
        self.custom_size = Some(size);
        self
    }

}

impl egui::Widget for SelectableLabel {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Self {
            selected,
            text,
            custom_size,
            fill,
            rounding,
            stroke,
        } = self;

        let button_padding = ui.spacing().button_padding;
        let total_extra = button_padding + button_padding;

        let wrap_width = ui.available_width() - total_extra.x;
        let text = text.into_galley(ui, None, wrap_width, egui::TextStyle::Button);

        let mut desired_size;
        match custom_size {
            Some(size) => desired_size = size,
            None => desired_size = total_extra + text.size(),
        }
        desired_size.y = desired_size.y.max(ui.spacing().interact_size.y);

        let (rect, response) = ui.allocate_at_least(desired_size, egui::Sense::click());
        response.widget_info(|| {
            egui::WidgetInfo::selected(egui::WidgetType::SelectableLabel, selected, text.text())
        });

        if ui.is_rect_visible(rect) {
            let text_pos = ui
                .layout()
                .align_size_within_rect(text.size(), rect.shrink2(button_padding))
                .min;

            let visuals = ui.style().interact_selectable(&response, selected);

            if selected || response.hovered() || response.has_focus() {
                let rect = rect.expand(visuals.expansion);

                ui.painter()
                    .rect(rect, rounding, fill, stroke);
            }
            text.paint_with_visuals(ui.painter(), text_pos, &visuals);
        }
        response
    }
}
