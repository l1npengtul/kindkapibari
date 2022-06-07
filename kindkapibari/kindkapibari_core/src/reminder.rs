use chrono::{naive::NaiveTime, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct OneTimeReminder {
    pub name: String,
    pub set: DateTime<Utc>,
    pub expire: DateTime<Utc>,
}

pub const MONDAY: u32 = 0;
pub const TUESDAY: u32 = 1;
pub const WEDNESDAY: u32 = 2;
pub const THURSDAY: u32 = 3;
pub const FRIDAY: u32 = 4;
pub const SATURDAY: u32 = 5;
pub const SUNDAY: u32 = 6;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Weekdays {
    #[default]
    Everyday,
    Days([bool; 7]),
}

// inspired by the alarm app on my phone
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct RecurringReminder {
    pub time: NaiveTime,
    pub days: Weekdays,
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Reminders {
    pub one_time: Vec<OneTimeReminder>,
    pub recurring: Vec<RecurringReminder>,
}
