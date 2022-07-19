use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Component)]
pub struct Speed {
    pub max_speed: u32,
    pub acceleration: u32,
    pub deceleration: u32,
}
