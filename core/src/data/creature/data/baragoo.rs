use crate::data::creature::def::item_drop::CreatureItemDrop;
use crate::data::creature::def::tier::CreatureTier;
use crate::data::creature::def::CreatureDefinition;
use crate::data::item::id::ItemID;

pub static CREATURE_BARAGOO: CreatureDefinition = CreatureDefinition {
    name: "Baragoo",
    max_power: 25,
    breeding_cooldown: 120,
    tier: CreatureTier::Abundant,
    item_drops: &[
        CreatureItemDrop {
            item_id: ItemID::GooGoo,
            min_proficiency: 0.5,
            drop_chance: 1.0,
            count_range: &(2..=10),
        },
        CreatureItemDrop {
            item_id: ItemID::Goober,
            min_proficiency: 0.75,
            drop_chance: 0.5,
            count_range: &(2..=10),
        },
        CreatureItemDrop {
            item_id: ItemID::RefinedGoober,
            min_proficiency: 0.9,
            drop_chance: 0.2,
            count_range: &(1..=5),
        },
    ],
    sprite_png: include_bytes!("../../../../../assets/creatures/Forest Noxluff.png"),
};
