use crate::actions::report::GameActionReport;
use crate::actions::GameActions;
use crate::state::GameState;
use crate::update_report::GameUpdateReport;
use serde::{Deserialize, Serialize};

pub mod actions;
pub mod data;
mod error;
pub mod state;
mod systems;
pub mod types;
mod update_report;
mod utils;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Game {
    pub state: GameState,
    #[serde(skip, default)]
    pub actions: GameActions,
}

impl Game {
    pub fn update(&mut self) -> GameUpdateReport {
        let now = std::time::Instant::now();

        let mut action_report = GameActionReport::default();
        for action in self.actions.take_actions() {
            match self.state.handle_action(action) {
                Ok(feedback) => action_report.on_feedback(feedback),
                Err(error) => action_report.on_error(error),
            }
        }
        let state_report = self.state.update();

        let time_elapsed = now.elapsed();

        GameUpdateReport {
            action_report,
            state_report,
            time_elapsed,
        }
    }
}
