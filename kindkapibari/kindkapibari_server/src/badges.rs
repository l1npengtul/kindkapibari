use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum Badge {
    AlphaTester(u64),
    EarlyAdopter(i64), // UTC Timestamp of join date
    Verified,
    Developer,
    Supporter([u8; 3]),
    PengChanApproved,
}
