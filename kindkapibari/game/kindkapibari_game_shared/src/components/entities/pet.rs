use crate::components::{
    entities::{health::Health, state::PetState},
    place::Location,
};
use bevy::prelude::{Bundle, Component};
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
pub struct PetInteractable;

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
pub struct Affection {
    pub level_scaling_factor_multiplier: f32,
    pub experience: f32,
    pub max_experience: f32,
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
pub struct Talkable; // TODO: Add talking pets (coconoutpak)

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize, Bundle)]
pub struct PetBundle {
    pub health: Health,
    pub state: PetState,
    #[bundle]
    pub location: Location,
}
