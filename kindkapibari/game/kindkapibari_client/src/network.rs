use bevy::prelude::EventReader;
use kindkapibari_game_shared::{protocol::Protocol, resources::connection::ChannelIndex};
use naia_bevy_client::{events::MessageEvent, Client};
use serde::{Deserialize, Serialize};

pub fn on_connected(client: Client<Protocol, ChannelIndex>) {}
pub fn on_disconnected(client: Client<Protocol, ChannelIndex>) {}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct NetworkInterpolation {
    pub network_interp_num_pkts: u32,
    pub network_events: Vec<Protocol>,
}

pub fn network_event(
    mut event_reader: EventReader<MessageEvent<Protocol, ChannelIndex>>,
    client: Client<Protocol, ChannelIndex>,
) {
    for net_event in event_reader.iter() {
        match net_event {}
    }
}
