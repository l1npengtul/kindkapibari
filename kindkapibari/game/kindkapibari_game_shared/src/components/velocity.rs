use bevy::prelude::{Component, UVec3};
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
pub struct Velocity {
    pub vel: UVec3,
}
