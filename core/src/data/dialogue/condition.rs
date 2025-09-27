use crate::state::GameState;
use crate::types::flag::GameFlag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum DialogueCondition {
    FlagSet(GameFlag),
    FlagUnset(GameFlag),
}

impl DialogueCondition {
    pub fn check(&self, state: &GameState) -> bool {
        match self {
            Self::FlagSet(flag) => state.flags.get(*flag),
            Self::FlagUnset(flag) => !state.flags.get(*flag),
        }
    }
}
