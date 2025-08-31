use crate::data::creature::data::*;
use crate::data::creature::def::CreatureDefinition;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, EnumIter)]
pub enum CreatureID {
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
