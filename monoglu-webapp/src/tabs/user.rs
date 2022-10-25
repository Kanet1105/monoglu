use crate::cell::{Grid, Cell};

pub struct User {
    grid: Grid,
    selected_achor: String,
}

impl User {
    pub fn new() -> Self {
        Self { 
            grid: Grid::new("user_tab_grid", 4, 5),
            selected_achor: "A".to_owned(),
        }
    }
}

impl super::Tab for User {
    fn name(&self) -> &'static str {
        "User"
    }

    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                egui::widgets::global_dark_light_mode_switch(ui);
                let list = [("a", "A"), ("b", "B"), ("c", "C")];
                for (name, anchor) in list {
                    if ui.add(
                        CustomSelectableLabel::new(self.selected_achor == anchor, name)
                            .custom_size(egui::vec2(50.0, 30.0))
                            .fill(egui::Color32::GOLD)
                            .stroke(1.0, egui::Color32::DARK_RED)
                            .rounding(egui::epaint::Rounding::same(25.0))

                        ).clicked()
                    {
                        self.selected_achor = anchor.to_owned();
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.grid.show(ctx);
        });
    }
}

//-------------------------------------------------------------
// Custom Selectable Label
pub struct CustomSelectableLabel {
    selected: bool,
    text: egui::WidgetText,
    custom_size: Option<egui::Vec2>,
    fill: egui::Color32,
    rounding: egui::epaint::Rounding,
    stroke: egui::Stroke,
}

impl CustomSelectableLabel {
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

impl egui::Widget for CustomSelectableLabel {
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
