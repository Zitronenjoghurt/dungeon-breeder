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
            min_proficiency: 0.75,
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
    item_drops: &[
        CreatureItemDrop {
            item_id: ItemID::SlimyDabs,
            min_proficiency: 0.5,
            drop_chance: 1.0,
            count_range: &(2..=20),
        },
        CreatureItemDrop {
            item_id: ItemID::SlimyJelly,
            min_proficiency: 0.75,
            drop_chance: 0.5,
            count_range: &(1..=5),
        },
        CreatureItemDrop {
            item_id: ItemID::SlimeAmethyst,
            min_proficiency: 0.95,
            drop_chance: 0.1,
            count_range: &(1..=1),
        },
    ],
    sprite_png: include_bytes!("../../../../assets/creatures/Slimei.png"),
};

pub static CREATURE_BARAGOO: CreatureDefinition = CreatureDefinition {
    name: "Baragoo",
    max_power: 25,
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
    sprite_png: include_bytes!("../../../../assets/creatures/Forest Noxluff.png"),
};

pub static CREATURE_TINKY: CreatureDefinition = CreatureDefinition {
    name: "Tinky",
    max_power: 30,
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
    sprite_png: include_bytes!("../../../../assets/creatures/Forest Mobun.png"),
};
