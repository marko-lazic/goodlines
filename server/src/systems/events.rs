use crate::global::Global;
use crate::user::{GoodUser, UserState};
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Res, ResMut};
use bevy_log::info;
use common::channels::Channels;
use common::protocol::{EntityAssignment, Protocol};
use naia_bevy_server::{
    events::{AuthorizationEvent, ConnectionEvent, DisconnectionEvent, MessageEvent},
    Server,
};

pub fn authorization_event(
    mut event_reader: EventReader<AuthorizationEvent<Protocol>>,
    mut server: Server<Protocol, Channels>,
    mut user_state: ResMut<UserState>,
) {
    for event in event_reader.iter() {
        if let AuthorizationEvent(user_key, Protocol::Auth(auth)) = event {
            if UserState::is_authorized(&*auth.username, &*auth.password) {
                // Accept incoming connection
                server.accept_connection(user_key);
                user_state.create(GoodUser::new(*user_key, &*auth.username));
            } else {
                // Reject incoming connection
                server.reject_connection(user_key);
            }
        }
    }
}

pub fn connection_event<'world, 'state>(
    mut event_reader: EventReader<ConnectionEvent>,
    global: Res<Global>,
    mut user_state: ResMut<UserState>,
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

        // Add user entity to user state
        if let Some(user) = user_state.find_mut(&*user_key) {
            user.entity = Option::from(entity);
        }

        // Send an Entity Assignment message to the User that owns the Square
        let mut assignment_message = EntityAssignment::new(true);
        assignment_message.entity.set(&server, &entity);

        server.send_message(user_key, Channels::EntityAssignment, &assignment_message);
    }
}

pub fn disconnection_event(
    mut event_reader: EventReader<DisconnectionEvent>,
    global: Res<Global>,
    mut user_state: ResMut<UserState>,
    mut server: Server<Protocol, Channels>,
) {
    for event in event_reader.iter() {
        let DisconnectionEvent(user_key, user) = event;
        info!("Goodlines Server disconnected from: {:?}", user.address);

        if let Some(user) = user_state.remove(user_key) {
            server
                .entity_mut(&user.entity.unwrap())
                .leave_room(&global.main_room_key)
                .despawn();
        }
    }
}

pub fn receive_message_event(
    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,
    mut global: ResMut<Global>,
    server: Server<Protocol, Channels>,
) {
    for event in event_reader.iter() {
        if let MessageEvent(_user_key, Channels::SendMessage, Protocol::Message(message)) = event {
            info!(
                "[{}] {}",
                message.username.to_string(),
                message.text.to_string()
            );

            if let Some(entity) = &message.entity.get(&server) {
                global
                    .last_entity_message_command
                    .insert(*entity, message.clone());
            }
        }
    }
}

pub fn broadcast_message_event<'world, 'state>(
    mut global: ResMut<Global>,
    user_state: Res<UserState>,
    mut server: Server<'world, 'state, Protocol, Channels>,
) {
    // Broadcast to all users
    for (_entity, last_message) in global.last_entity_message_command.drain() {
        for (user_key, _user) in user_state.iter() {
            server.send_message(user_key, Channels::BroadcastMessage, &last_message.clone());
        }
    }
}

pub fn tick(mut server: Server<Protocol, Channels>) {
    // All game logic should happen here, on a tick event
    //info!("tick");

    // Update scopes of entities
    for (_, user_key, entity) in server.scope_checks() {
        // You'd normally do whatever checks you need to in here..
        // to determine whether each Entity should be in scope or not.

        // This indicates the Entity should be in this scope.
        server.user_scope(&user_key).include(&entity);

        // And call this if Entity should NOT be in this scope.
        // server.user_scope(..).exclude(..);
    }

    // This is very important! Need to call this to actually send all update packets
    // to all connected Clients!
    server.send_all_updates();
}
