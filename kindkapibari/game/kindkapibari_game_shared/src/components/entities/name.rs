use bevy::prelude::{Component, Transform};
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
pub struct HiddenName {
    pub name: String,
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
pub struct DisplayedName {
    pub offset: Transform,
    pub name: String,
}
