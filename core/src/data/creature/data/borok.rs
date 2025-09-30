use crate::data::creature::def::item_drop::CreatureItemDrop;
use crate::data::creature::def::tier::CreatureTier;
use crate::data::creature::def::CreatureDefinition;
use crate::data::item::id::ItemID;

pub static CREATURE_BOROK: CreatureDefinition = CreatureDefinition {
    name: "Borok",
    flavor_text: "Despite its appearance, it is not dangerous if you dont threaten it. They like brutally fighting with each other for fun and are loyal protectors to whoever they deem worthy of being their master.",
    max_power: 400,
    breeding_cooldown: 6000,
    summoning_cooldown: None,
    tier: CreatureTier::Abundant,
    item_drops: &[
        CreatureItemDrop {
            item_id: ItemID::RockySpike,
            min_proficiency: 0.5,
            drop_chance: 1.0,
            count_range: &(1..=4),
        },
        CreatureItemDrop {
            item_id: ItemID::BoulderedFist,
            min_proficiency: 0.75,
            drop_chance: 0.5,
            count_range: &(1..=2),
        },
        CreatureItemDrop {
            item_id: ItemID::GlowingRedPebbledEye,
            min_proficiency: 0.95,
            drop_chance: 0.2,
            count_range: &(1..=1),
        },
    ],
    sprite_png: include_bytes!("../../../../../assets/creatures/Rock golem.png"),
};
