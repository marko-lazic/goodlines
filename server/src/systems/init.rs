use crate::global::Global;
use bevy_ecs::entity::Entity;
use bevy_ecs::system::Commands;
use bevy_log::info;
use common::channels::Channels;
use common::protocol::{Message, Protocol};
use naia_bevy_server::{Server, ServerAddrs};
use std::collections::HashMap;

pub fn init(mut commands: Commands, mut server: Server<Protocol, Channels>) {
    info!("Goodlines Server Demo is running");

    // Naia Server initialization
    let server_addresses = ServerAddrs::new(
        "127.0.0.1:14191"
            .parse()
            .expect("could not parse Signaling address/port"),
        // IP Address to listen on for UDP WebRTC data channels
        "127.0.0.1:14192"
            .parse()
            .expect("could not parse WebRTC data address/port"),
        // The public WebRTC IP address to advertise
        "http://127.0.0.1:14192",
    );

    server.listen(&server_addresses);

    // Create a new, singular room, which will contain Users and Entities that they
    // can receive updates from
    let main_room_key = server.make_room().key();

    // Resources
    commands.insert_resource(Global {
        main_room_key,
        last_entity_message_command: HashMap::<Entity, Message>::new(),
    })
}
