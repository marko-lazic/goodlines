use crate::App;
use bevy::app::Plugin;

use crate::connection::SendMessageEvent;
use crate::global::Global;
use bevy::prelude::{EventWriter, ResMut};
use bevy_egui::egui::{Context, FontData, FontDefinitions, FontFamily, ScrollArea};
use bevy_egui::{egui, EguiContext};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::configure_visuals);
        app.add_system(Self::ui_update);
    }
}

impl UiPlugin {
    fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
        Self::configure_fonts(egui_ctx.ctx_mut());
    }

    fn ui_update(
        mut egui_context: ResMut<EguiContext>,
        mut global: ResMut<Global>,
        mut ev_send_message: EventWriter<SendMessageEvent>,
    ) {
        egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
            ScrollArea::vertical()
                .stick_to_bottom()
                .always_show_scroll(true)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for a in &global.messages {
                        ui.horizontal(|ui| {
                            let msg = format!("[{}]: {}", &a.username, &a.text);
                            ui.label(msg);
                        });
                    }
                    ui.label("");
                });
        });

        egui::TopBottomPanel::bottom("footer").show(egui_context.ctx_mut(), |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.add_space(1.0);
                let input_widget = egui::TextEdit::singleline(&mut global.input_text)
                    .hint_text("Enter your message here...")
                    .desired_width(480.0);
                let edit_re = ui.add(input_widget);
                if edit_re.ctx.input().key_pressed(egui::Key::Enter) {
                    global.send_message(&mut ev_send_message);
                    edit_re.request_focus();
                }

                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    let send_btn_widget = egui::Button::new("Send");
                    let send_btn_re =
                        ui.add_enabled(global.is_something_written(), send_btn_widget);

                    if send_btn_re.clicked() {
                        global.send_message(&mut ev_send_message);
                        edit_re.request_focus();
                    }
                });
                ui.add_space(1.0);
            });
        });
    }

    fn configure_fonts(ctx: &Context) {
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
}
