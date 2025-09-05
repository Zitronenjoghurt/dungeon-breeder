use crate::data::creature::def::item_drop::CreatureItemDrop;
use crate::data::creature::def::tier::CreatureTier;
use crate::data::creature::def::CreatureDefinition;
use crate::data::item::id::ItemID;

pub static CREATURE_GONK: CreatureDefinition = CreatureDefinition {
    name: "Gonk",
    max_power: 10,
    tier: CreatureTier::Abundant,
    item_drops: &[
        CreatureItemDrop {
            item_id: ItemID::Gonkball,
            min_proficiency: 0.5,
            drop_chance: 1.0,
            count_range: &(1..=3),
        },
        CreatureItemDrop {
            item_id: ItemID::GonkCrystal,
            min_proficiency: 0.8,
            drop_chance: 0.5,
            count_range: &(1..=3),
        },
        CreatureItemDrop {
            item_id: ItemID::GonkSoul,
            min_proficiency: 0.95,
            drop_chance: 0.1,
            count_range: &(1..=1),
        },
    ],
    sprite_png: include_bytes!("../../../../assets/creatures/Ice Snowball.png"),
};

pub static CREATURE_SLIME: CreatureDefinition = CreatureDefinition {
    name: "Slime",
    max_power: 20,
    tier: CreatureTier::Abundant,
    item_drops: &[CreatureItemDrop {
        item_id: ItemID::SlimyJelly,
        min_proficiency: 0.5,
        drop_chance: 1.0,
        count_range: &(2..=10),
    }],
    sprite_png: include_bytes!("../../../../assets/creatures/Slimei.png"),
};
