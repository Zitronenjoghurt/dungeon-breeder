use crate::data::creature::def::item_drop::CreatureItemDrop;
use crate::data::creature::def::tier::CreatureTier;
use crate::data::creature::def::CreatureDefinition;
use crate::data::item::id::ItemID;

pub static CREATURE_SLIME: CreatureDefinition = CreatureDefinition {
    name: "Slime",
    flavor_text: "This seems like just a boring amalgamation of slimy substance, but its actually quite resilient. It appears to be able to thrive in any environment.",
    max_power: 25,
    breeding_cooldown: 40,
    summoning_cooldown: None,
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
    sprite_png: include_bytes!("../../../../../assets/creatures/Slimei.png"),
};
