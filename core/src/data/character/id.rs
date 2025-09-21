use crate::data::character::data::*;
use crate::data::character::def::CharacterDefinition;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
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
pub enum CharacterID {
    #[default]
    Advisor,
}

impl CharacterID {
    pub fn iter_def() -> impl Iterator<Item = &'static CharacterDefinition> {
        Self::iter().map(|id| id.def())
    }

    pub const fn def(self) -> &'static CharacterDefinition {
        match self {
            Self::Advisor => &CHARACTER_ADVISOR,
        }
    }
}
