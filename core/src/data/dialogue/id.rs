use crate::data::dialogue::data::*;
use crate::data::dialogue::Dialogue;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum_macros::EnumIter;

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    Deserialize,
    EnumIter,
)]
pub enum DialogueID {
    #[default]
    Tutorial,
}

impl DialogueID {
    pub fn get_dialogue(self) -> Dialogue {
        let mut builder = Dialogue::builder();

        builder = match self {
            DialogueID::Tutorial => build_tutorial(builder),
        };

        builder.build()
    }
}

impl Display for DialogueID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
