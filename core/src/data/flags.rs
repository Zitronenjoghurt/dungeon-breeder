use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(
    Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize, EnumIter,
)]
#[repr(u16)]
pub enum GameFlag {
    TutorialComplete = 0,
    UnlockedCompendium = 1,
    UnlockedStatistics = 2,
    UnlockedSpecimenOverview = 3,
    UnlockedItems = 4,
    UnlockedDungeon = 5,
    UnlockedBreeding = 6,
    UnlockedFusion = 7,
}
