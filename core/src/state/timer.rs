use serde::{Deserialize, Serialize};

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
}
