use crate::App;
use bevy::app::Plugin;
use bevy::log::info;
use bevy::prelude::EventReader;
use common::channels::Channels;
use common::protocol::{Message, Protocol};
use naia_bevy_client::Client;

pub struct SendMessageEvent(pub String);

pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SendMessageEvent>();
        app.add_startup_system(Self::init);
        app.add_system(Self::send_message_event);
    }
}

impl ConnectionPlugin {
    fn init(mut client: Client<Protocol, Channels>) {
        info!("Goodlines chat started");
        client.connect("http://127.0.0.1:14191");
    }

    fn send_message_event(
        mut client: Client<Protocol, Channels>,
        mut events: EventReader<SendMessageEvent>,
    ) {
        for event in events.iter() {
            let message = Message::new("todo", event.0.as_str());
            client.send_message(Channels::SendMessage, &message);
        }
    }
}
