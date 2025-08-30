use crate::creature::tier::CreatureTier;
use crate::creature::Creature;

pub const CREATURE_DATA: [Creature; 1] = [Creature {
    id: 0,
    name: "Gonk",
    max_power: 1,
    tier: CreatureTier::Abundant,
}];
