use chrono::{DateTime, Utc};
use crossbeam::atomic::AtomicCell;
use std::sync::atomic::{AtomicU16, Ordering};

#[derive(Debug)]
pub struct SnowflakeIdGenerator {
    epoch: DateTime<Utc>,
    last: AtomicCell<DateTime<Utc>>,
    sequence: AtomicU16,
    machine_id: u8,
}

impl SnowflakeIdGenerator {
    #[must_use]
    pub fn new(epoch: DateTime<Utc>, machine_id: u8) -> Option<Self> {
        let now = Utc::now();
        if epoch >= now {
            return None;
        }
        Some(Self {
            epoch,
            last: AtomicCell::new(epoch),
            sequence: AtomicU16::default(),
            machine_id,
        })
    }

    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_lossless)]
    pub fn generate_id(&self) -> u64 {
        let sequence = self.sequence.load(Ordering::SeqCst);
        let now = Utc::now();
        let epoch = self.epoch;
        let last_gen = self.last.load();

        let difference = now - epoch;
        if now == last_gen {
            self.sequence.fetch_add(1, Ordering::SeqCst);
        }
        if now > last_gen {
            self.sequence.store(0, Ordering::SeqCst);
            self.last.store(now);
        }

        // store 44 bits of time, 7 bits of machine ID, 16 bits of sequence
        (difference.num_milliseconds() as u64) << 22
            | (self.machine_id as u64) << 16 // FIXME: Check if this is right (i am dumb shit)
            | (sequence as u64)
    }
}
