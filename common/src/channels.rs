use naia_shared::{derive_channels, Channel, ChannelDirection, ChannelMode, ReliableSettings};

#[derive_channels]
pub enum Channels {
    SendMessage,
    BroadcastMessage,
    EntityAssignment,
}

pub const CHANNEL_CONFIG: &[Channel<Channels>] = &[
    Channel {
        index: Channels::SendMessage,
        direction: ChannelDirection::ClientToServer,
        mode: ChannelMode::UnorderedReliable(ReliableSettings::default()),
    },
    Channel {
        index: Channels::BroadcastMessage,
        direction: ChannelDirection::ServerToClient,
        mode: ChannelMode::UnorderedReliable(ReliableSettings::default()),
    },
    Channel {
        index: Channels::EntityAssignment,
        direction: ChannelDirection::ServerToClient,
        mode: ChannelMode::UnorderedReliable(ReliableSettings::default()),
    },
];
