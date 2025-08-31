use crate::actions::GameActions;
use crate::data::GameData;
use crate::state::GameState;
use serde::{Deserialize, Serialize};

pub mod actions;
pub mod creature;
pub mod data;
pub mod state;
mod utils;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Game {
    pub state: GameState,
    #[serde(skip, default)]
    pub data: GameData,
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
