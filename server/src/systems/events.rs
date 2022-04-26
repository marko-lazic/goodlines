use bevy_ecs::event::EventReader;
use bevy_log::info;
use common::channels::Channels;
use common::protocol::Protocol;
use naia_bevy_server::events::MessageEvent;

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
