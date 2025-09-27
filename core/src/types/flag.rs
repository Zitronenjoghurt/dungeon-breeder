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
#[repr(u16)]
pub enum GameFlag {
    #[default]
    TutorialComplete = 0,
    UnlockedCompendium = 1,
    UnlockedStatistics = 2,
    UnlockedSpecimenOverview = 3,
    UnlockedItems = 4,
    UnlockedDungeon = 5,
    UnlockedBreeding = 6,
    UnlockedFusion = 7,
    UnlockedTopBar = 8,
    OfflineProgressionReportEnabled = 9,
    HasClickedSpecimenOverview = 10,
}

impl Display for GameFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
