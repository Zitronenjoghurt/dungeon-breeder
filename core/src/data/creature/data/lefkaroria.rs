use crate::data::creature::def::item_drop::CreatureItemDrop;
use crate::data::creature::def::tier::CreatureTier;
use crate::data::creature::def::CreatureDefinition;
use crate::data::item::id::ItemID;

pub static CREATURE_LEFKARORIA: CreatureDefinition = CreatureDefinition {
    name: "Lefkaroria",
    flavor_text: "A shark-like creature which can float through the air like its weightless. It homes in on its prey like a torpedo.",
    max_power: 100,
    breeding_cooldown: 3000,
    summoning_cooldown: None,
    tier: CreatureTier::Abundant,
    item_drops: &[
        CreatureItemDrop {
            item_id: ItemID::LefkaTush,
            min_proficiency: 0.5,
            drop_chance: 1.0,
            count_range: &(5..=20),
        },
        CreatureItemDrop {
            item_id: ItemID::LefkaFin,
            min_proficiency: 0.75,
            drop_chance: 0.75,
            count_range: &(1..=5),
        },
        CreatureItemDrop {
            item_id: ItemID::FocusedVoid,
            min_proficiency: 0.95,
            drop_chance: 0.15,
            count_range: &(1..=3),
        },
    ],
    sprite_png: include_bytes!("../../../../../assets/creatures/Sea Shark.png"),
};
