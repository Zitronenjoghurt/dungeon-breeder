use crate::data::creature::def::item_drop::CreatureItemDrop;
use crate::data::creature::def::tier::CreatureTier;
use crate::data::creature::def::CreatureDefinition;
use crate::data::item::id::ItemID;

pub static CREATURE_UGGROOT: CreatureDefinition = CreatureDefinition {
    name: "Uggroot",
    max_power: 45,
    breeding_cooldown: 600,
    summoning_cooldown: None,
    tier: CreatureTier::Abundant,
    item_drops: &[
        CreatureItemDrop {
            item_id: ItemID::Vegebit,
            min_proficiency: 0.5,
            drop_chance: 1.0,
            count_range: &(5..=20),
        },
        CreatureItemDrop {
            item_id: ItemID::Carrotooth,
            min_proficiency: 0.75,
            drop_chance: 0.6,
            count_range: &(1..=5),
        },
        CreatureItemDrop {
            item_id: ItemID::StinkyCarrot,
            min_proficiency: 0.95,
            drop_chance: 0.1,
            count_range: &(1..=3),
        },
    ],
    sprite_png: include_bytes!("../../../../../assets/creatures/Forest Fangrot.png"),
};
