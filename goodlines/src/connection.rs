use crate::App;
use bevy::app::Plugin;
use bevy::log::info;
use bevy::prelude::EventReader;
use common::channels::Channels;
use common::protocol::{Auth, Message, Protocol};
use naia_bevy_client::{Client, Stage};

pub struct SendMessageEvent(pub Message);

pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SendMessageEvent>();
        app.add_startup_system(Self::init);
        app.add_system_to_stage(Stage::Connection, Self::connect_event);
        app.add_system_to_stage(Stage::Disconnection, Self::disconnect_event);
        app.add_system_to_stage(Stage::Tick, Self::send_message_event);
    }
}

impl ConnectionPlugin {
    fn init(mut client: Client<Protocol, Channels>) {
        info!("Goodlines chat started");
        client.auth(Auth::new("test", "12345"));
        client.connect("http://127.0.0.1:14191");
    }

    pub fn connect_event(client: Client<Protocol, Channels>) {
        info!("Client connected to: {}", client.server_address());
    }

    pub fn disconnect_event(client: Client<Protocol, Channels>) {
        info!("Client disconnected from: {}", client.server_address());
    }

    fn send_message_event(
        mut client: Client<Protocol, Channels>,
        mut events: EventReader<SendMessageEvent>,
    ) {
        for event in events.iter() {
            
            client.send_message(Channels::SendMessage, &event.0);
        }
    }
}
