use crate::data::dialogue::data::*;
use crate::data::dialogue::Dialogue;
use serde::{Deserialize, Serialize};
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
