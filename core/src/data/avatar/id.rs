use crate::data::avatar::data::*;
use crate::data::avatar::def::AvatarDefinition;
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
pub enum AvatarID {
    #[default]
    Advisor,
    Mayor,
    Developer,
}

impl AvatarID {
    pub fn iter_def() -> impl Iterator<Item = &'static AvatarDefinition> {
        Self::iter().map(|id| id.def())
    }

    pub const fn def(self) -> &'static AvatarDefinition {
        match self {
            Self::Advisor => &AVATAR_ADVISOR,
            Self::Mayor => &AVATAR_MAYOR,
            Self::Developer => &AVATAR_DEVELOPER,
        }
    }
}

impl Display for AvatarID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
