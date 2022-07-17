use bevy::prelude::{Bundle, Quat, Transform, TransformBundle};
use serde::{Deserialize, Serialize};
use std::path::Component;

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
pub struct Location {
    pub rotation: Quat,
    #[bundle]
    pub transform: TransformBundle,
}
