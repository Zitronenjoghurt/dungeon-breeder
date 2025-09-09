use crate::state::update_report::GameStateUpdateReport;
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
    pub fn on_game_state_update(&mut self, report: &GameStateUpdateReport) {
        if report.ticks_elapsed <= 10 {
            self.active_ticks = self.active_ticks.saturating_add(report.ticks_elapsed);
        } else {
            self.passive_ticks = self.active_ticks.saturating_add(report.ticks_elapsed);
        }

        self.times_specimen_slain = self
            .times_specimen_slain
            .saturating_add(report.times_specimen_slain);
    }

    pub fn on_successful_breed(&mut self) {
        self.times_bred = self.times_bred.saturating_add(1);
    }

    pub fn on_successful_fusion(&mut self) {
        self.times_fused = self.times_fused.saturating_add(1);
    }
}
