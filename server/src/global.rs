use bevy_ecs::entity::Entity;
use common::protocol::Message;
use naia_bevy_server::RoomKey;
use std::collections::HashMap;

pub struct Global {
    pub main_room_key: RoomKey,
    pub last_entity_message_command: HashMap<Entity, Message>,
}
