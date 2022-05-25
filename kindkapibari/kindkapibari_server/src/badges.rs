use kindkapibari_core::{impl_attr_err, AttrString};
use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, AttrString,
)]
pub enum Badge {
    AlphaTester(u64),
    EarlyAdopter(u64), // UTC Timestamp of join date
    Verified,
    Developer,
    Contributor,
    Supporter([u8; 3]),
    PengChanApproved,
}

impl_attr_err!(Badge);
