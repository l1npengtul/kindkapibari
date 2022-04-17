use chrono::{DateTime, Utc};
use crossbeam::atomic::AtomicCell;
use std::{
    sync::atomic::{AtomicU32, Ordering},
    thread::sleep,
    time::Duration,
};

const U17_MAX: u32 = 2_u32 ^ 17;

pub struct SnowflakeIdGenerator {
    epoch: DateTime<Utc>,
    last: AtomicCell<DateTime<Utc>>,
    sequence: AtomicU32,
}

impl SnowflakeIdGenerator {
    pub fn new(epoch: DateTime<Utc>) -> Option<Self> {
        let now = Utc::now();
        if epoch >= now {
            return None;
        }
        Some(Self {
            epoch,
            last: AtomicCell::new(epoch),
            sequence: Default::default(),
        })
    }

    pub fn generate_id(&self, machine: u8) -> Option<u64> {
        let sequence = self.sequence.load(Ordering::SeqCst);
        let now = Utc::now();
        let epoch = self.epoch;
        let last_gen = self.last.load();

        if epoch < now || last_gen < now || machine > 32 {
            return None;
        }
        let difference = now - epoch;
        if now == last_gen {
            self.sequence.fetch_add(1, Ordering::SeqCst);
        }
        if now > last_gen {
            self.sequence.store(0, Ordering::SeqCst);
            self.last.store(now);
        }

        // store 44 bits of time, 5 bits of machine ID, 17 bits of sequence
        Some(
            (difference.num_milliseconds() as u64) << 22
                | (machine as u64) << 17
                | (sequence as u64),
        )
    }
}
