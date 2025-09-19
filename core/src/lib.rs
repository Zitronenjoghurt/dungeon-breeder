use crate::actions::GameActions;
use crate::events::GameEvents;
use crate::state::GameState;
use crate::update_report::GameUpdateReport;
use serde::{Deserialize, Serialize};

pub mod actions;
pub mod data;
mod error;
mod events;
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
    #[serde(skip, default)]
    pub events: GameEvents,
}

impl Game {
    pub fn update(&mut self) -> GameUpdateReport {
        let now = std::time::Instant::now();

        let action_report = self.actions.handle(&mut self.state, &mut self.events);
        let state_report = self.state.update();

        let time_elapsed = now.elapsed();

        GameUpdateReport {
            action_report,
            state_report,
            time_elapsed,
        }
    }
}
