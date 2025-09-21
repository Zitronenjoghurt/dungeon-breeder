use crate::data::dialogue::data::*;
use crate::data::dialogue::Dialogue;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum_macros::EnumIter;

#[derive(
    Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize, EnumIter,
)]
pub enum DialogueID {
    Tutorial,
}

impl DialogueID {
    pub fn get_dialogue(self) -> &'static Dialogue<'static> {
        match self {
            DialogueID::Tutorial => &DIALOGUE_TUTORIAL,
        }
    }
}

impl Display for DialogueID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
