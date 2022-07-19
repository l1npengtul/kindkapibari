use crate::components::{
    entities::{health::Health, state::PlayerState},
    place::Location,
    speed::Speed,
    velocity::Velocity,
};
use bevy::prelude::Bundle;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize, Bundle)]
pub struct PlayerBundle {
    pub health: Health,
    pub state: PlayerState,
    #[bundle]
    pub location: Location,
    pub velocity: Velocity,
    pub speed: Speed,
}
