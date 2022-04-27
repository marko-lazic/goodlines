pub struct Message {
    pub username: String,
    pub text: String,
}

pub struct Global {
    pub messages: Vec<Message>,
    pub username: String,
    pub input_text: String,
}

impl Default for Global {
    fn default() -> Self {
        let iter = (0..20).map(|a| Message {
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

    pub fn send_message(&mut self) {
        if self.is_something_written() {
            self.messages.push(Message {
                username: self.username.to_string(),
                text: self.input_text.to_string(),
            });
            // clear out chat input
            self.input_text = "".to_string();
        }
    }
}
