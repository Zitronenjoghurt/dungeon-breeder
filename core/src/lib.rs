use crate::actions::GameActions;
use crate::state::GameState;
use serde::{Deserialize, Serialize};

pub mod actions;
pub mod data;
pub mod state;
mod systems;
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
            self.state.handle_action(action);
        }
    }
}
