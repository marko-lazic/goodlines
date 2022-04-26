pub struct Message {
    pub username: String,
    pub text: String,
}

pub struct Client {
    pub username: String,
    pub input_text: String,
}

pub struct GoodLines {
    pub messages: Vec<Message>,
    pub client: Client,
}

impl Default for GoodLines {
    fn default() -> Self {
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

impl GoodLines {
    pub fn is_something_written(&self) -> bool {
        !self.client.input_text.trim().is_empty()
    }

    pub fn send_message(&mut self) {
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
