use crate::creature::def::tier::CreatureTier;
use crate::creature::def::CreatureDefinition;

pub static CREATURE_GONK: CreatureDefinition = CreatureDefinition {
    name: "Gonk",
    max_power: 10,
    tier: CreatureTier::Abundant,
};

pub static CREATURE_SLIME: CreatureDefinition = CreatureDefinition {
    name: "Slime",
    max_power: 20,
    tier: CreatureTier::Abundant,
};
