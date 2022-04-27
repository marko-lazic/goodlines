use crate::connection::SendMessageEvent;
use bevy::prelude::EventWriter;
use common::protocol::Message;

pub struct UiMessage {
    pub username: String,
    pub text: String,
}

pub struct Global {
    pub messages: Vec<UiMessage>,
    pub username: String,
    pub input_text: String,
}

impl Default for Global {
    fn default() -> Self {
        let iter = (0..20).map(|a| UiMessage {
            username: format!("username{}", a),
            text: format!("message{}", a),
        });
        Global {
            messages: Vec::from_iter(iter),
            input_text: "".to_string(),
            username: "Me".to_string(),
        }
    }
}

impl Global {
    pub fn is_something_written(&self) -> bool {
        !self.input_text.trim().is_empty()
    }

    pub fn send_message(&mut self, ev_send_message: &mut EventWriter<SendMessageEvent>) {
        if self.is_something_written() {
            self.messages.push(UiMessage {
                username: self.username.to_string(),
                text: self.input_text.to_string(),
            });

            ev_send_message.send(SendMessageEvent(Message::new(
                self.username.as_str(),
                self.input_text.as_str(),
            )));

            // clear out chat input
            self.input_text = "".to_string();
        }
    }
}
