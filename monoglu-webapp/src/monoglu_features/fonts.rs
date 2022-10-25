/// setup Engilsh / Korean fonts / emojis
pub fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::empty();

    fonts.font_data.insert(
        "Roboto-Regular".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "..\\..\\assets\\fonts\\Roboto-Regular.ttf"
        )),
    );

    fonts.font_data.insert(
        "NotoSansKR-Regular".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "..\\..\\assets\\fonts\\NotoSansKR-Regular.otf"
        )),
    );

    // Some good looking emojis. Use as first priority:
    fonts.font_data.insert(
        "NotoEmoji-Regular".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "..\\..\\assets\\fonts\\NotoEmoji-Regular.ttf"
        ))
        .tweak(
            egui::FontTweak {
                scale: 0.81,           // make it smaller
                y_offset_factor: -0.2, // move it up
                y_offset: 0.0,
            },
        ),
    );

    // Bigger emojis, and more. <http://jslegers.github.io/emoji-icon-font/>:
    fonts.font_data.insert(
        "emoji-icon-font".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "..\\..\\assets\\fonts\\emoji-icon-font.ttf"
        ))
        .tweak(
            egui::FontTweak {
                scale: 0.88,           // make it smaller
                y_offset_factor: 0.07, // move it down slightly
                y_offset: 0.0,
            },
        ),
    );


    fonts
        .families
        .insert(
            egui::FontFamily::Proportional,
            vec![
                "Roboto-Regular".to_owned(),
                "NotoSansKR-Regular".to_owned(),
                "NotoEmoji-Regular".to_owned(),
                "emoji-icon-font".to_owned(),
            ],
        );

    fonts
        .families
        .insert(
            egui::FontFamily::Monospace,
            vec![
                "Roboto-Regular".to_owned(),
                "NotoSansKR-Regular".to_owned(),
                "emoji-icon-font".to_owned(),
            ], 
        );

    ctx.set_fonts(fonts);
}