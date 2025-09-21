use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(
    Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize, EnumIter,
)]
#[repr(u16)]
pub enum GameFlag {
    TutorialComplete,
}
