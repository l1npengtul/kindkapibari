use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum Badge {
    AlphaTester,
    EarlyAdopter,
    Verified,
    Developer,
    Supporter([u8; 3]),
    HRT,
    PengChanApproved,
}
