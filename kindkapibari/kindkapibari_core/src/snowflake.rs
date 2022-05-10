use chrono::{DateTime, Utc};
use crossbeam::atomic::AtomicCell;
use std::sync::atomic::{AtomicU16, Ordering};

pub struct SnowflakeIdGenerator {
    epoch: DateTime<Utc>,
    last: AtomicCell<DateTime<Utc>>,
    sequence: AtomicU16,
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

        if epoch < now || last_gen < now || machine > 64 {
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

        // store 44 bits of time, 7 bits of machine ID, 16 bits of sequence
        Some(
            (difference.num_milliseconds() as u64) << 22
                | (machine as u64) << 16 // FIXME: Check if this is right (i am dumb shit)
                | (sequence as u64),
        )
    }
}
