use serde::{Deserialize, Serialize};

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
