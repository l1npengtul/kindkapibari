use crate::components::entities::{pet::PetBundle, player::PlayerBundle};
use bevy::prelude::{Component, Quat};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Component)]
#[non_exhaustive]
pub enum EntityType {
    Pet(PetBundle),
    Player(PlayerBundle),
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Component)]
pub struct EntityUpdate {
    pub ecs_id: u64,
    pub entity_type: EntityType,
}
