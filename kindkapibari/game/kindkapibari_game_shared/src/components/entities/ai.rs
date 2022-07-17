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
pub struct AggressiveAi;

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
pub struct PassiveAggressiveAi {
    pub angry: bool,
}

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
pub struct PassiveAi {
    pub scared: bool,
}

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
pub struct FriendlyAi;
