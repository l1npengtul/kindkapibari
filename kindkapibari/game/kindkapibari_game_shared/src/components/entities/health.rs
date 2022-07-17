use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Hash,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Component,
)]
pub struct Health {
    pub points: u32,
    pub max_pts: u32,
    pub extra: u32,
    pub max_extra: u32,
}
