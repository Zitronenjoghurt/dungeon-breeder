use crate::data::item::data::*;
use crate::data::item::def::ItemDefinition;
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
pub enum ItemID {
    #[default]
    Gonkball = 0,
    GonkCrystal = 1,
    GonkSoul = 2,
    SlimyDabs = 3,
    SlimyJelly = 4,
    SlimeAmethyst = 5,
    GooGoo = 6,
    Goober = 7,
    RefinedGoober = 8,
    Tinkofuzz = 9,
    Tinkosphere = 10,
    Tinkolite = 11,
    Vegebit = 12,
    Carrotooth = 13,
    StinkyCarrot = 14,
    VoidFragment = 15,
    VoidShard = 16,
    FocusedVoid = 17,
    LefkaTush = 18,
    LefkaFin = 19,
    LefkaPlate = 20,
    RockySpike = 21,
    BoulderedFist = 22,
    GlowingRedPebbledEye = 23,
}

impl ItemID {
    pub fn iter_def() -> impl Iterator<Item = &'static ItemDefinition> {
        Self::iter().map(|id| id.def())
    }

    pub const fn def(self) -> &'static ItemDefinition {
        match self {
            Self::Gonkball => &ITEM_GONKBALL,
            Self::GonkCrystal => &ITEM_GONK_CRYSTAL,
            Self::GonkSoul => &ITEM_GONK_SOUL,
            Self::SlimyDabs => &ITEM_SLIMY_DABS,
            Self::SlimyJelly => &ITEM_SLIMY_JELLY,
            Self::SlimeAmethyst => &ITEM_SLIME_AMETHYST,
            Self::GooGoo => &ITEM_GOO_GOO,
            Self::Goober => &ITEM_GOOBER,
            Self::RefinedGoober => &ITEM_REFINED_GOOBER,
            Self::Tinkofuzz => &ITEM_TINKOFUZZ,
            Self::Tinkosphere => &ITEM_TINKOSPHERE,
            Self::Tinkolite => &ITEM_TINKOLITE,
            Self::Vegebit => &ITEM_VEGEBIT,
            Self::Carrotooth => &ITEM_CARROTOOTH,
            Self::StinkyCarrot => &ITEM_STINKY_CARROT,
            Self::VoidFragment => &ITEM_VOID_FRAGMENT,
            Self::VoidShard => &ITEM_VOID_SHARD,
            Self::FocusedVoid => &ITEM_FOCUSED_VOID,
            Self::LefkaTush => &ITEM_LEFKA_TUSH,
            Self::LefkaFin => &ITEM_LEFKA_FIN,
            Self::LefkaPlate => &ITEM_LEFKA_PLATE,
            Self::RockySpike => &ITEM_ROCKY_SPIKE,
            Self::BoulderedFist => &ITEM_BOULDERED_FIST,
            Self::GlowingRedPebbledEye => &ITEM_GLOWING_RED_PEBBLED_EYE,
        }
    }
}

impl Display for ItemID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
