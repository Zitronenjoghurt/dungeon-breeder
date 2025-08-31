use crate::creature::tier::CreatureTier;
use crate::creature::Creature;

pub const CREATURE_DATA: &[Creature] = &[
    Creature {
        id: 0,
        name: "Gonk",
        max_power: 10,
        tier: CreatureTier::Abundant,
    },
    Creature {
        id: 1,
        name: "Slime",
        max_power: 20,
        tier: CreatureTier::Abundant,
    },
];
