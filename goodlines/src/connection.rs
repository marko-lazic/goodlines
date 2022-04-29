use crate::global::{OwnedEntity, UiMessage};
use crate::util::random_user_credentials;
use crate::{App, Global};
use bevy::app::Plugin;
use bevy::log::info;
use bevy::prelude::{Commands, EventReader, ResMut};
use common::channels::Channels;
use common::protocol::{Auth, Protocol};
use naia_bevy_client::events::MessageEvent;
use naia_bevy_client::{Client, CommandsExt, Stage};

pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::init);
        app.add_system_to_stage(Stage::Connection, Self::connect_event);
        app.add_system_to_stage(Stage::Disconnection, Self::disconnect_event);
        app.add_system_to_stage(Stage::ReceiveEvents, Self::receive_message_event);
        app.add_system_to_stage(Stage::Tick, Self::send_message);
    }
}

impl ConnectionPlugin {
    fn init(mut client: Client<Protocol, Channels>, mut global: ResMut<Global>) {
        info!("Goodlines chat started");

        let (username, password) = random_user_credentials();
        client.auth(Auth::new(username, password));
        client.connect("http://127.0.0.1:14191");
        global.username = username.to_string();
    }

    pub fn connect_event(client: Client<Protocol, Channels>) {
        info!("Client connected to: {}", client.server_address());
    }

    pub fn disconnect_event(client: Client<Protocol, Channels>) {
        info!("Client disconnected from: {}", client.server_address());
    }

    fn send_message(mut global: ResMut<Global>, mut client: Client<Protocol, Channels>) {
        while global.has_messages_to_be_sent() {
            if let Some(mut message) = global.pop_message() {
                if let Some(owned_entity) = &global.owned_entity {
                    message.entity.set(&client, &owned_entity.confirmed);
                    client.send_message(Channels::SendMessage, &message);
                }
            }
        }
    }

    fn receive_message_event(
        mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,
        mut local: Commands,
        mut global: ResMut<Global>,
        client: Client<Protocol, Channels>,
    ) {
        for event in event_reader.iter() {
            if let MessageEvent(Channels::BroadcastMessage, Protocol::Message(message)) = event {
                global.messages.push(UiMessage::new(&*message));
            }

            if let MessageEvent(
                Channels::EntityAssignment,
                Protocol::EntityAssignment(entity_assign),
            ) = event
            {
                let assign = *entity_assign.assign;

                let entity = entity_assign.entity.get(&client).unwrap();
                if assign {
                    info!("gave ownership of entity");

                    let prediction_entity =
                        CommandsExt::<Protocol>::duplicate_entity(&mut local, entity).id();

                    global.owned_entity = Some(OwnedEntity::new(entity, prediction_entity));
                } else {
                    let mut disowned: bool = false;
                    if let Some(owned_entity) = &global.owned_entity {
                        if owned_entity.confirmed == entity {
                            local.entity(owned_entity.predicted).despawn();
                            disowned = true;
                        }
                    }
                    if disowned {
                        info!("removed ownership of entity");
                        global.owned_entity = None;
                    }
                }
            }
        }
    }
}
