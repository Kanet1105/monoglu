// setup verious text size
#[inline]
fn too_big() -> egui::TextStyle {
    egui::TextStyle::Name("TooBig".into())
}

#[inline]
fn heading3() -> egui::TextStyle {
    egui::TextStyle::Name("ContextHeading".into())
}

pub fn configure_text_styles(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (egui::TextStyle::Heading, egui::FontId::new(25.0, egui::FontFamily::Proportional)),
        (too_big(), egui::FontId::new(30.0, egui::FontFamily::Proportional)),
        (heading3(), egui::FontId::new(19.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Body, egui::FontId::new(16.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Monospace, egui::FontId::new(12.0, egui::FontFamily::Monospace)),
        (egui::TextStyle::Button, egui::FontId::new(12.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Small, egui::FontId::new(8.0, egui::FontFamily::Proportional)),
    ]
    .into();
    ctx.set_style(style);
}