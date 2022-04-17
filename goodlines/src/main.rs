use eframe::egui::{Context, FontData, FontDefinitions, FontFamily, ScrollArea, Vec2};
use eframe::epi::{App, Frame, Storage};
use eframe::{egui, run_native, NativeOptions};

const APP_WIDTH: f32 = 546.0;
const APP_HEIGHT: f32 = 364.0;

struct Message {
    username: String,
    text: String,
}

struct Client {
    username: String,
    input_text: String,
}

struct GoodLines {
    messages: Vec<Message>,
    client: Client,
}

impl GoodLines {
    pub(crate) fn configure_fonts(&self, ctx: &Context) {
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

    fn is_something_written(&self) -> bool {
        !self.client.input_text.trim().is_empty()
    }

    fn send_message(&mut self) {
        if self.is_something_written() {
            self.messages.push(Message {
                username: self.client.username.to_string(),
                text: self.client.input_text.to_string(),
            });
            // clear out chat input
            self.client.input_text = "".to_string();
        }
    }
}

impl GoodLines {
    fn new() -> GoodLines {
        let iter = (0..20).map(|a| Message {
            username: format!("username{}", a),
            text: format!("message{}", a),
        });
        GoodLines {
            messages: Vec::from_iter(iter),
            client: Client {
                input_text: "".to_string(),
                username: "Me".to_string(),
            },
        }
    }
}

impl App for GoodLines {
    fn setup(&mut self, ctx: &Context, _frame: &Frame, _storage: Option<&dyn Storage>) {
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &Context, frame: &Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical()
                .stick_to_bottom()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for a in &self.messages {
                        ui.horizontal(|ui| {
                            let msg = format!("[{}]: {}", &a.username, &a.text);
                            ui.label(msg);
                        });
                    }
                    ui.label("");
                });

            egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.add_space(1.0);
                    let input_widget = egui::TextEdit::singleline(&mut self.client.input_text)
                        .hint_text("Enter your message here...")
                        .desired_width(480.0);
                    let edit_re = ui.add(input_widget);
                    if edit_re.ctx.input().key_pressed(egui::Key::Enter) {
                        self.send_message();
                        edit_re.request_focus();
                    }

                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                        let send_btn_widget = egui::Button::new("Send");
                        let send_btn_re =
                            ui.add_enabled(self.is_something_written(), send_btn_widget);

                        if send_btn_re.clicked() {
                            self.send_message();
                            edit_re.request_focus();
                        }
                    });
                    ui.add_space(1.0);
                });
            });
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }

    fn name(&self) -> &str {
        "Good Lines"
    }
}

fn main() {
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(APP_WIDTH, APP_HEIGHT));
    run_native(Box::new(GoodLines::new()), win_option);
}
