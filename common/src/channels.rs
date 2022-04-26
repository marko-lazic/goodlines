use naia_shared::{derive_channels, Channel, ChannelDirection, ChannelMode, ReliableSettings};

#[derive_channels]
pub enum Channels {
    SendMessage,
    PullMessage,
}

pub const CHANNEL_CONFIG: &[Channel<Channels>] = &[
    Channel {
        index: Channels::SendMessage,
        direction: ChannelDirection::ClientToServer,
        mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
    },
    Channel {
        index: Channels::PullMessage,
        direction: ChannelDirection::ServerToClient,
        mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
    },
];
