use crate::data::item::data::*;
use crate::data::item::def::ItemDefinition;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(
    Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize, EnumIter,
)]
pub enum ItemID {
    Gonkball,
}

impl ItemID {
    pub fn iter_def() -> impl Iterator<Item = &'static ItemDefinition> {
        Self::iter().map(|id| id.def())
    }

    pub const fn def(self) -> &'static ItemDefinition {
        match self {
            Self::Gonkball => &ITEM_GONKBALL,
        }
    }
}
