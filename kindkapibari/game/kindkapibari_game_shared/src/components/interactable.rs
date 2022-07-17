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
pub struct DescriptionInteractable {
    pub description: String,
}