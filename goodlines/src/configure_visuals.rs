use crate::egui::{Context, FontData, FontDefinitions, FontFamily};

pub(crate) fn configure_fonts(ctx: &Context) {
    // create font def object.. load font.. set size of different text sizes.. load font
    let mut font_def = FontDefinitions::default();
    font_def.font_data.insert(
        "MesloLGS".to_owned(),
        FontData::from_static(include_bytes!("../../MesloLGS_NF_Regular.ttf")),
    );
    // font_def.families.insert(eframe::egui::TextStyle::Heading, (FontFamily::Proportional, 35.0));

    font_def
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "MesloLGS".to_owned());

    ctx.set_fonts(font_def);
}
