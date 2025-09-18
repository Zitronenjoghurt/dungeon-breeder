use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum CreatureTier {
    Abundant,
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythical,
    Almighty,
}

impl Display for CreatureTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
