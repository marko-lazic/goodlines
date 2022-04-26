use bevy_ecs::prelude::Component;

use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Message {
    pub username: Property<String>,
    pub text: Property<String>,
}

impl Message {
    pub fn new(username: &str, text: &str) -> Self {
        Message::new_complete(username.to_string(), text.to_string())
    }
}
