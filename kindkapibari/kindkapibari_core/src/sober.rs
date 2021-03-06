use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// const SOBER_SCHEMA: u64 = 0;
// const SOBERS_SCHEMA: u64 = 0;

// represent as a </3?
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(utoipa::Component))]
pub struct Sober {
    pub id: u64,
    pub name: String,
    pub start_time: DateTime<Utc>,
}

impl Default for Sober {
    fn default() -> Self {
        Self {
            id: 0,
            name: "".to_string(),
            start_time: Utc::now(),
        }
    }
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(utoipa::Component))]
pub struct Sobers {
    pub sobers: Vec<Sober>,
}

#[cfg(feature = "server")]
crate::impl_sea_orm!(Sober, Sobers);
#[cfg(feature = "server")]
crate::impl_redis!(Sober, Sobers);
