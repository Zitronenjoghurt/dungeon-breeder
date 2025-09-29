use chrono::{DateTime, TimeDelta, Utc};
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
    #[tracing::instrument(
        target = "game",
        name = "game::clock::update",
        level = "trace",
        skip(self)
    )]
    pub fn update(&mut self) -> u64 {
        let now = Utc::now();
        let elapsed = now - self.last_tick;

        let seconds = elapsed.num_seconds();
        if seconds <= 0 {
            0
        } else {
            self.last_tick += chrono::Duration::seconds(seconds);
            elapsed.num_seconds() as u64
        }
    }

    pub fn roll_back(&mut self, seconds: u64) {
        self.last_tick -= TimeDelta::seconds(seconds as i64);
    }
}
