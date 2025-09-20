use crate::events::event::GameEvent;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameStatistics {
    pub started_at: DateTime<Utc>,
    /// Ticks the player actively played
    pub active_ticks: u64,
    /// Idle ticks
    pub passive_ticks: u64,
    pub times_bred: u64,
    pub times_fused: u64,
    pub times_specimen_slain: u64,
}

impl Default for GameStatistics {
    fn default() -> Self {
        GameStatistics {
            started_at: Utc::now(),
            active_ticks: 0,
            passive_ticks: 0,
            times_bred: 0,
            times_fused: 0,
            times_specimen_slain: 0,
        }
    }
}

impl GameStatistics {
    #[tracing::instrument(
        target = "game",
        name = "game::state::statistics::handle_event",
        level = "trace",
        skip(self)
    )]
    pub fn handle_event(&mut self, event: &GameEvent) {
        match event {
            GameEvent::SpecimenBred(_) => self.times_bred = self.times_bred.saturating_add(1),
            GameEvent::SpecimenFused(_) => self.times_fused = self.times_fused.saturating_add(1),
            GameEvent::SpecimenSlain(_) => {
                self.times_specimen_slain = self.times_specimen_slain.saturating_add(1)
            }
            _ => {}
        }
    }

    #[tracing::instrument(
        target = "game",
        name = "game::state::statistics::on_ticks_elapsed",
        level = "trace",
        skip(self)
    )]
    pub fn on_ticks_elapsed(&mut self, ticks: u64) {
        if ticks <= 10 {
            self.active_ticks = self.active_ticks.saturating_add(ticks);
        } else {
            self.passive_ticks = self.passive_ticks.saturating_add(ticks);
        }
    }
}
