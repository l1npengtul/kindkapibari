use crate::protocol::{entity::EntityUpdate, map::LoadMapUpdate};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct TotalGameStateUpdate {
    pub entities: Vec<EntityUpdate>,
    pub map: Vec<LoadMapUpdate>,
}
