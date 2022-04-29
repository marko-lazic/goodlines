use bevy::prelude::Entity;
use common::protocol::Message;
use std::collections::VecDeque;

pub struct UiMessage {
    pub username: String,
    pub text: String,
}

impl UiMessage {
    pub fn new(message: &Message) -> Self {
        Self {
            username: message.username.to_string(),
            text: message.text.to_string(),
        }
    }
}

pub struct OwnedEntity {
    pub confirmed: Entity,
    pub predicted: Entity,
}

impl OwnedEntity {
    pub fn new(confirmed_entity: Entity, predicted_entity: Entity) -> Self {
        OwnedEntity {
            confirmed: confirmed_entity,
            predicted: predicted_entity,
        }
    }
}

pub struct Global {
    pub owned_entity: Option<OwnedEntity>,
    pub messages: Vec<UiMessage>,
    pub username: String,
    pub input_text: String,
    to_be_sent: VecDeque<Message>,
}

impl Default for Global {
    fn default() -> Self {
        let iter = (0..20).map(|a| UiMessage {
            username: format!("username{}", a),
            text: format!("message{}", a),
        });
        Global {
            owned_entity: None,
            messages: Vec::from_iter(iter),
            input_text: "".to_string(),
            username: "Me".to_string(),
            to_be_sent: VecDeque::new(),
        }
    }
}

impl Global {
    pub fn is_something_written(&self) -> bool {
        !self.input_text.trim().is_empty()
    }

    pub fn send_message(&mut self) {
        if self.is_something_written() {
            self.to_be_sent.push_back(Message::new(
                self.username.as_str(),
                self.input_text.as_str(),
            ));

            // clear out chat input
            self.input_text = "".to_string();
        }
    }

    pub fn pop_message(&mut self) -> Option<Message> {
        self.to_be_sent.pop_front()
    }

    pub fn has_messages_to_be_sent(&self) -> bool {
        !self.to_be_sent.is_empty()
    }
}
