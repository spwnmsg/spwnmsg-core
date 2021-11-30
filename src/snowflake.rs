use chrono::Utc;
use parking_lot::Mutex;
use std::sync::Arc;

pub struct Snowflake {
    epoch: i64,
    worker_id: i64,
    datacenter_id: i64,
    sequence: i64,
    time: Arc<Mutex<i64>>,
}

impl Default for Snowflake {
    fn default() -> Snowflake {
        Snowflake {
            epoch: 1_573_948_800,
            worker_id: 1,
            datacenter_id: 1,
            sequence: 0,
            time: Arc::new(Mutex::new(0)),
        }
    }
}

impl Snowflake {
    pub fn new(epoch: i64, worker_id: i64, datacenter_id: i64) -> Snowflake {
        Default::default()
    }

    pub fn generate(&mut self) -> i64 {
        let mut last_timestamp = self.time.lock();
        let mut timestamp = self.get_time();
        if timestamp == *last_timestamp {
            self.sequence = (self.sequence + 1) & (-1 ^ (-1 << 12));
            if self.sequence == 0 && timestamp <= *last_timestamp {
                timestamp = self.get_time();
            }
        } else {
            self.sequence = 0;
        }
        *last_timestamp = timestamp;
        (timestamp << 22) | (self.worker_id << 17) | (self.datacenter_id << 12) | self.sequence
    }

    pub fn generate_u8_u64(&mut self) -> [u8; 8] {
        self.generate().to_le_bytes()
    }

    fn get_time(&self) -> i64 {
        Utc::now().timestamp_millis() - self.epoch
    }
}