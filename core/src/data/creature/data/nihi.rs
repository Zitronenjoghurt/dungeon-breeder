use crate::data::creature::def::item_drop::CreatureItemDrop;
use crate::data::creature::def::tier::CreatureTier;
use crate::data::creature::def::CreatureDefinition;
use crate::data::item::id::ItemID;

pub static CREATURE_NIHI: CreatureDefinition = CreatureDefinition {
    name: "Nihi",
    flavor_text: "A doll-shaped being which is hard to read. Its staring at whatever is closest to it, but rarely ever moves an inch. It does not seem to be hostile, but you can never be sure.",
    max_power: 70,
    breeding_cooldown: 1500,
    summoning_cooldown: None,
    tier: CreatureTier::Abundant,
    item_drops: &[
        CreatureItemDrop {
            item_id: ItemID::VoidFragment,
            min_proficiency: 0.5,
            drop_chance: 1.0,
            count_range: &(3..=10),
        },
        CreatureItemDrop {
            item_id: ItemID::VoidShard,
            min_proficiency: 0.75,
            drop_chance: 0.5,
            count_range: &(3..=5),
        },
        CreatureItemDrop {
            item_id: ItemID::FocusedVoid,
            min_proficiency: 0.95,
            drop_chance: 0.15,
            count_range: &(1..=3),
        },
    ],
    sprite_png: include_bytes!("../../../../../assets/creatures/Voidoll Erebia.png"),
};
