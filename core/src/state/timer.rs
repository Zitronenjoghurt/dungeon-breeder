use humantime::format_duration;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Timer(u64);

impl Timer {
    pub fn tick(&mut self, max_ticks: u64) -> bool {
        self.0 += 1;

        if self.0 >= max_ticks {
            self.0 = 0;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.0 = 0;
    }

    pub fn progress(&self, max_ticks: u64) -> f32 {
        self.0 as f32 / max_ticks as f32
    }

    pub fn format_time_left(&self, max_ticks: u64) -> String {
        let current_secs = self.0;
        let max_secs = max_ticks;
        let secs_left = max_secs - current_secs;
        format_duration(Duration::from_secs(secs_left)).to_string()
    }
}
