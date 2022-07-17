use naia_shared::{
    derive_channels, Channel, ChannelDirection, ChannelMode, ReliableSettings, TickBufferSettings,
};

#[derive_channels]
pub enum ChannelIndex {
    PlayerCommand,
    ServerUpdate,
}

pub const CHANNELS: &[Channel<ChannelIndex>] = &[
    Channel {
        index: ChannelIndex::PlayerCommand,
        mode: ChannelMode::TickBuffered(TickBufferSettings::default()),
        direction: ChannelDirection::ClientToServer,
    },
    Channel {
        index: ChannelIndex::ServerUpdate,
        mode: ChannelMode::UnorderedReliable(ReliableSettings::default()),
        direction: ChannelDirection::ServerToClient,
    },
];

pub struct Connection {}
