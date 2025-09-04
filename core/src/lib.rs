use crate::actions::GameActions;
use crate::state::GameState;
use serde::{Deserialize, Serialize};

pub mod actions;
pub mod data;
mod error;
pub mod state;
mod systems;
pub mod types;
mod utils;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Game {
    pub state: GameState,
    #[serde(skip, default)]
    pub actions: GameActions,
}

impl Game {
    pub fn update(&mut self) {
        for action in self.actions.take_actions() {
            // ToDo: Give successful + failed messages
            let _result = self.state.handle_action(action);
        }

        // ToDo: Give feedback on what was updated, e.g. for report when starting game after a while
        self.state.update();
    }
}
