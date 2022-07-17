use bevy::{log::error, prelude::Component};
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
pub enum Facing {
    Forward,
    LSide,
    RSide,
    Backward,
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
pub enum Heading {
    North, // North is away from the camera(aka showing the back)
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Heading {
    pub fn facing(&self) -> Facing {
        match self {
            Heading::North | Heading::NorthEast | Heading::NorthWest => Facing::Forward,
            Heading::South | Heading::SouthEast | Heading::SouthWest => Facing::Backward,
            Heading::East => Facing::RSide,
            Heading::West => Facing::LSide,
        }
    }
}

impl From<Heading> for Facing {
    fn from(h: Heading) -> Self {
        h.facing()
    }
}
