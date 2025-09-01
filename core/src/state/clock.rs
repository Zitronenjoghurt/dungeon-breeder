use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Clock {
    pub last_tick: DateTime<Utc>,
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            last_tick: Utc::now(),
        }
    }
}

impl Clock {
    pub fn update(&mut self) -> u64 {
        let now = Utc::now();
        let elapsed = now - self.last_tick;

        if elapsed.num_seconds() <= 0 {
            0
        } else {
            self.last_tick = now;
            elapsed.num_seconds() as u64
        }
    }
}
