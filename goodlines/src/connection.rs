use crate::{App, Global};
use bevy::app::Plugin;
use bevy::log::info;
use bevy::prelude::ResMut;
use common::channels::Channels;
use common::protocol::{Auth, Protocol};
use naia_bevy_client::{Client, Stage};

pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::init);
        app.add_system_to_stage(Stage::Connection, Self::connect_event);
        app.add_system_to_stage(Stage::Disconnection, Self::disconnect_event);
        app.add_system_to_stage(Stage::Tick, Self::send_message);
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

    fn send_message(mut global: ResMut<Global>, mut client: Client<Protocol, Channels>) {
        while global.has_messages_to_be_sent() {
            if let Some(message) = global.pop_message() {
                client.send_message(Channels::SendMessage, &message);
            }
        }
    }
}
