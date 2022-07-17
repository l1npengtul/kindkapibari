use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct LoadMapUpdate {
    pub ecs_id: u64,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct MapBuildUpdated {
    pub ecs_id: u64,
}
