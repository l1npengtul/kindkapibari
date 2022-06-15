use chrono::{naive::NaiveTime, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct OneTimeReminder {
    pub name: String,
    pub set: DateTime<Utc>,
    pub expire: DateTime<Utc>,
}

impl Default for OneTimeReminder {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            set: Utc::now(),
            expire: Utc::now(),
        }
    }
}

pub const MONDAY: u32 = 0;
pub const TUESDAY: u32 = 1;
pub const WEDNESDAY: u32 = 2;
pub const THURSDAY: u32 = 3;
pub const FRIDAY: u32 = 4;
pub const SATURDAY: u32 = 5;
pub const SUNDAY: u32 = 6;

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Weekdays {
    #[default]
    Everyday,
    Days([bool; 7]),
}

// inspired by the alarm app on my phone
#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct RecurringReminder {
    pub time: NaiveTime,
    pub days: Weekdays,
}

impl Default for RecurringReminder {
    fn default() -> Self {
        Self {
            time: Utc::now().time(),
            days: Weekdays::Everyday,
        }
    }
}

#[derive(Clone, Debug, Default, Hash, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Reminders {
    pub one_time: Vec<OneTimeReminder>,
    pub recurring: Vec<RecurringReminder>,
}
