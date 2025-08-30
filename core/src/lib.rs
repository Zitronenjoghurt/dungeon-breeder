use crate::creature::library::CreatureLibrary;
use crate::creature::Creature;
use crate::state::GameState;
use serde::{Deserialize, Serialize};

pub mod creature;
mod state;
mod utils;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Game {
    pub state: GameState,
    #[serde(skip, default)]
    creature_library: CreatureLibrary,
}

impl Game {
    pub fn get_creature_by_id(&self, id: u16) -> Option<&Creature> {
        self.creature_library.get_by_id(id)
    }
}
