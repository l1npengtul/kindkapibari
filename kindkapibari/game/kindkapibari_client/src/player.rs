use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Component)]
pub struct LocalPlayer {}
