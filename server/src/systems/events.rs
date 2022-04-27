use crate::global::Global;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::ResMut;
use bevy_log::info;
use common::channels::Channels;
use common::protocol::Protocol;
use naia_bevy_server::{
    events::{AuthorizationEvent, ConnectionEvent, DisconnectionEvent, MessageEvent},
    Server,
};

pub fn authorization_event(
    mut event_reader: EventReader<AuthorizationEvent<Protocol>>,
    mut server: Server<Protocol, Channels>,
) {
    for event in event_reader.iter() {
        if let AuthorizationEvent(user_key, Protocol::Auth(auth)) = event {
            if *auth.username == "test" && *auth.password == "12345" {
                // Accept incoming connection
                server.accept_connection(user_key);
            } else {
                // Reject incoming connection
                server.reject_connection(user_key);
            }
        }
    }
}

pub fn connection_event<'world, 'state>(
    mut event_reader: EventReader<ConnectionEvent>,
    mut global: ResMut<Global>,
    mut server: Server<'world, 'state, Protocol, Channels>,
) {
    for event in event_reader.iter() {
        let ConnectionEvent(user_key) = event;
        let address = server
            .user_mut(&user_key)
            // Add User to the main Room
            .enter_room(&global.main_room_key)
            // Get User's address for logging
            .address();

        info!("Goodlines Server connected to: {}", address);

        // Spawn entity
        let entity = server.spawn().enter_room(&global.main_room_key).id();

        global.user_entity_map.insert(*user_key, entity);
    }
}

pub fn disconnection_event(
    mut event_reader: EventReader<DisconnectionEvent>,
    mut global: ResMut<Global>,
    mut server: Server<Protocol, Channels>,
) {
    for event in event_reader.iter() {
        let DisconnectionEvent(user_key, user) = event;
        info!("Goodlines Server disconnected from: {:?}", user.address);

        if let Some(entity) = global.user_entity_map.remove(user_key) {
            server
                .entity_mut(&entity)
                .leave_room(&global.main_room_key)
                .despawn();
        }
    }
}

pub fn receive_message_event(mut event_reader: EventReader<MessageEvent<Protocol, Channels>>) {
    for event in event_reader.iter() {
        if let MessageEvent(_user_key, Channels::SendMessage, Protocol::Message(message)) = event {
            info!(
                "[{}] {}",
                message.username.to_string(),
                message.text.to_string()
            );
        }
    }
}
