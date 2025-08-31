use crate::data::creature::def::item_drop::CreatureItemDrop;
use crate::data::creature::def::tier::CreatureTier;
use crate::data::creature::def::CreatureDefinition;
use crate::data::item::id::ItemID;

pub static CREATURE_GONK: CreatureDefinition = CreatureDefinition {
    name: "Gonk",
    max_power: 10,
    tier: CreatureTier::Abundant,
    item_drops: &[CreatureItemDrop {
        item_id: ItemID::Gonkball,
        min_proficiency: 0.0,
        drop_chance: 1.0,
        count_range: &(1..=3),
    }],
};

pub static CREATURE_SLIME: CreatureDefinition = CreatureDefinition {
    name: "Slime",
    max_power: 20,
    tier: CreatureTier::Abundant,
    item_drops: &[],
};
