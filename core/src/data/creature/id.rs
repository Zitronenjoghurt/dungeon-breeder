use crate::data::creature::data::*;
use crate::data::creature::def::CreatureDefinition;
use crate::state::item::NewItem;
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
    Baragoo,
    Tinky,
}

impl CreatureID {
    pub fn iter_def() -> impl Iterator<Item = &'static CreatureDefinition> {
        Self::iter().map(|id| id.def())
    }

    pub const fn def(self) -> &'static CreatureDefinition {
        match self {
            Self::Gonk => &CREATURE_GONK,
            Self::Slime => &CREATURE_SLIME,
            Self::Baragoo => &CREATURE_BARAGOO,
            Self::Tinky => &CREATURE_TINKY,
        }
    }

    pub fn generate_drops(&self, proficiency: f32) -> Vec<NewItem> {
        let mut rng = rand::rng();
        self.def()
            .item_drops
            .iter()
            .filter_map(|item_drop| {
                let count = item_drop.generate_drop(&mut rng, proficiency)?;
                Some(NewItem {
                    item_id: item_drop.item_id,
                    amount: count as u64,
                })
            })
            .collect::<Vec<_>>()
    }
}

impl Display for CreatureID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
