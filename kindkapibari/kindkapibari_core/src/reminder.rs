use chrono::{naive::NaiveTime, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub struct OneTimeReminder {
    pub id: u64,
    pub name: String,
    pub set: DateTime<Utc>,
    pub expire: DateTime<Utc>,
}

impl Default for OneTimeReminder {
    fn default() -> Self {
        Self {
            id: 0,
            name: "".to_string(),
            set: Utc::now(),
            expire: Utc::now(),
        }
    }
}

pub const MONDAY: u8 = 0b0000_0001;
pub const TUESDAY: u8 = 0b0000_0010;
pub const WEDNESDAY: u8 = 0b0000_0100;
pub const THURSDAY: u8 = 0b0000_1000;
pub const FRIDAY: u8 = 0b0001_0000;
pub const SATURDAY: u8 = 0b0010_0000;
pub const SUNDAY: u8 = 0b0100_0000;

// inspired by the alarm app on my phone
#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Serialize, Deserialize)]
pub struct RecurringReminder {
    pub id: u64,
    pub name: String,
    pub time: NaiveTime,
    pub days: [bool; 7],
}

impl Default for RecurringReminder {
    fn default() -> Self {
        Self {
            id: 0,
            name: "".to_string(),
            time: Utc::now().time(),
            days: u8_bitflag_to_days(0),
        }
    }
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub struct OneTimeReminders {
    pub one_time: Vec<OneTimeReminder>,
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub struct RecurringReminders {
    pub recurring: Vec<RecurringReminder>,
}

#[must_use]
pub fn u8_bitflag_to_days(bitflag: u8) -> [bool; 7] {
    let monday = bitflag & MONDAY == MONDAY;
    let tuesday = bitflag & TUESDAY == TUESDAY;
    let wednesday = bitflag & WEDNESDAY == WEDNESDAY;
    let thursday = bitflag & THURSDAY == THURSDAY;
    let friday = bitflag & FRIDAY == FRIDAY;
    let saturday = bitflag & SATURDAY == SATURDAY;
    let sunday = bitflag & SUNDAY == SUNDAY;
    [
        monday, tuesday, wednesday, thursday, friday, saturday, sunday,
    ]
}

#[cfg(feature = "server")]
crate::impl_sea_orm!(RecurringReminders, OneTimeReminders);
#[cfg(feature = "server")]
crate::impl_redis!(RecurringReminders, OneTimeReminders);
