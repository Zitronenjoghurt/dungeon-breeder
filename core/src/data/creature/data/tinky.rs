use crate::data::creature::def::item_drop::CreatureItemDrop;
use crate::data::creature::def::tier::CreatureTier;
use crate::data::creature::def::CreatureDefinition;
use crate::data::item::id::ItemID;

pub static CREATURE_TINKY: CreatureDefinition = CreatureDefinition {
    name: "Tinky",
    flavor_text: "A fluffy fluffball which seems to enjoy having the crystal on its forehead rubbed. Touching its antenna feels strangely soothing (its unknown if this has any side effects).",
    max_power: 30,
    breeding_cooldown: 300,
    summoning_cooldown: None,
    tier: CreatureTier::Abundant,
    item_drops: &[
        CreatureItemDrop {
            item_id: ItemID::Tinkofuzz,
            min_proficiency: 0.5,
            drop_chance: 1.0,
            count_range: &(1..=15),
        },
        CreatureItemDrop {
            item_id: ItemID::Tinkosphere,
            min_proficiency: 0.75,
            drop_chance: 0.5,
            count_range: &(1..=5),
        },
        CreatureItemDrop {
            item_id: ItemID::Tinkolite,
            min_proficiency: 0.95,
            drop_chance: 0.1,
            count_range: &(1..=3),
        },
    ],
    sprite_png: include_bytes!("../../../../../assets/creatures/Forest Mobun.png"),
};
