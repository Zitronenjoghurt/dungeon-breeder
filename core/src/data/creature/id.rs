use crate::data::creature::data::*;
use crate::data::creature::def::CreatureDefinition;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
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
pub enum CreatureID {
    #[default]
    Gonk,
    Slime,
}

impl CreatureID {
    pub fn iter_def() -> impl Iterator<Item = &'static CreatureDefinition> {
        Self::iter().map(|id| id.def())
    }

    pub const fn def(self) -> &'static CreatureDefinition {
        match self {
            Self::Gonk => &CREATURE_GONK,
            Self::Slime => &CREATURE_SLIME,
        }
    }
}

impl Display for CreatureID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
