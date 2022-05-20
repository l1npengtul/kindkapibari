use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// const SOBER_SCHEMA: u64 = 0;
// const SOBERS_SCHEMA: u64 = 0;

// represent as a </3?
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct Sober {
    // pub schema: u64,
    pub name: String,
    pub time: DateTime<Utc>,
}

impl Default for Sober {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            time: Utc::now(),
        }
    }
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Serialize, Deserialize)]
pub struct Sobers {
    pub sobers: Vec<Sober>,
}

#[cfg(feature = "server")]
crate::impl_sea_orm!(Sober, Sobers);
