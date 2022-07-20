use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Component)]
pub struct Speed {
    pub max_speed: u32,
    pub acceleration: u32,
    pub deceleration: u32,
    pub run_multi: f32,
    pub crouch_multi: f32,
    pub low_hp_multi: f32,
    pub full_hp_multi: f32,
}
