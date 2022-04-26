mod configure_visuals;
mod goodlines;
mod settings;

use crate::goodlines::GoodLines;
use bevy::app::App;
use bevy::prelude::ResMut;
use bevy::DefaultPlugins;
use bevy_egui::egui::ScrollArea;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use configure_visuals::configure_fonts;

fn main() {
    App::new()
        .insert_resource(settings::window_descriptor())
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .init_resource::<GoodLines>()
        .add_startup_system(configure_visuals)
        .add_system(ui_update)
        .run();
}

fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    configure_fonts(egui_ctx.ctx_mut());
}

fn ui_update(mut egui_context: ResMut<EguiContext>, mut goodlines: ResMut<GoodLines>) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ScrollArea::vertical()
            .stick_to_bottom()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for a in &goodlines.messages {
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
            let input_widget = egui::TextEdit::singleline(&mut goodlines.client.input_text)
                .hint_text("Enter your message here...")
                .desired_width(480.0);
            let edit_re = ui.add(input_widget);
            if edit_re.ctx.input().key_pressed(egui::Key::Enter) {
                goodlines.send_message();
                edit_re.request_focus();
            }

            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                let send_btn_widget = egui::Button::new("Send");
                let send_btn_re = ui.add_enabled(goodlines.is_something_written(), send_btn_widget);

                if send_btn_re.clicked() {
                    goodlines.send_message();
                    edit_re.request_focus();
                }
            });
            ui.add_space(1.0);
        });
    });
}
