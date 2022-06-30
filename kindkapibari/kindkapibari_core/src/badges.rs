use kindkapibari_proc::AttrString;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, AttrString)]
pub enum Badge {
    AlphaTester(u64),
    EarlyAdopter(u64), // UTC Timestamp of join date
    Verified,
    Developer,
    Contributor,
    Supporter,
    PengChanApproved,
}
