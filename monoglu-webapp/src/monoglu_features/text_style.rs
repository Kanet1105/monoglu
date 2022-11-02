// setup verious text size

#[inline]
fn size42() -> egui::TextStyle {
    egui::TextStyle::Name("size42".into())
}

#[inline]
fn size38() -> egui::TextStyle {
    egui::TextStyle::Name("size38".into())
}

#[inline]
fn size32() -> egui::TextStyle {
    egui::TextStyle::Name("size32".into())
}

#[inline]
fn size24() -> egui::TextStyle {
    egui::TextStyle::Name("size24".into())
}

#[inline]
fn size20() -> egui::TextStyle {
    egui::TextStyle::Name("size20".into())
}

#[inline]
fn size18() -> egui::TextStyle {
    egui::TextStyle::Name("size18".into())
}

#[inline]
fn size14() -> egui::TextStyle {
    egui::TextStyle::Name("size14".into())
}

#[inline]
fn size12() -> egui::TextStyle {
    egui::TextStyle::Name("size12".into())
}

#[inline]
fn monospace32() -> egui::TextStyle {
    egui::TextStyle::Name("monospace32".into())
}

#[inline]
fn monospace24() -> egui::TextStyle {
    egui::TextStyle::Name("monospace24".into())
}

#[inline]
fn monospace16() -> egui::TextStyle {
    egui::TextStyle::Name("monospace16".into())
}

pub fn configure_text_styles(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (egui::TextStyle::Heading, egui::FontId::new(28.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Body, egui::FontId::new(16.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Monospace, egui::FontId::new(14.0, egui::FontFamily::Monospace)),
        (egui::TextStyle::Button, egui::FontId::new(14.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Small, egui::FontId::new(10.0, egui::FontFamily::Proportional)),
        (size42(), egui::FontId::new(42.0, egui::FontFamily::Proportional)),
        (size38(), egui::FontId::new(38.0, egui::FontFamily::Proportional)),
        (size32(), egui::FontId::new(32.0, egui::FontFamily::Proportional)),
        (size24(), egui::FontId::new(24.0, egui::FontFamily::Proportional)),
        (size20(), egui::FontId::new(20.0, egui::FontFamily::Proportional)),
        (size18(), egui::FontId::new(18.0, egui::FontFamily::Proportional)),
        (size14(), egui::FontId::new(14.0, egui::FontFamily::Proportional)),
        (size12(), egui::FontId::new(12.0, egui::FontFamily::Proportional)),
        (monospace32(), egui::FontId::new(32.0, egui::FontFamily::Monospace)),
        (monospace24(), egui::FontId::new(24.0, egui::FontFamily::Monospace)),
        (monospace16(), egui::FontId::new(26.0, egui::FontFamily::Monospace)),
    ]
    .into();
    ctx.set_style(style);
}