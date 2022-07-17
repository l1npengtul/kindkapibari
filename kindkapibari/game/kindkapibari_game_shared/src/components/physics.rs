use crate::components::place::Location;
use bevy::prelude::{Bundle, Component};
use bevy_rapier3d::{
    dynamics::RigidBody,
    geometry::{ActiveCollisionTypes, ActiveEvents, Collider},
};
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
pub struct Kinematic {
    pub acceleration: f32,
    pub base_speed: f32,
    pub current_speed: f32,
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
pub struct Rigid {
    pub weight: f32,
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
pub struct StaticObject;

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
    Bundle,
)]
pub struct KinematicObjectBundle {
    pub kinematic: Kinematic,
    pub active_collision_types: ActiveCollisionTypes,
    pub active_events: ActiveEvents,
    pub collider: Collider,
    pub rigid_body: RigidBody,
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
    Bundle,
)]
pub struct StaticObjectBundle {
    pub static_object: StaticObject,
    pub collider: Collider,
    pub rigid_body: RigidBody,
}
