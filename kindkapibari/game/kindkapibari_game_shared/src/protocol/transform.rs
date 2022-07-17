use crate::components::place::Location;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct TransformUpdate {
    pub entity_id: u64,
    pub location: Location,
}
