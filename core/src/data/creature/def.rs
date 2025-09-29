use crate::data::creature::def::item_drop::CreatureItemDrop;
use tier::CreatureTier;

pub mod item_drop;
pub mod tier;

#[derive(Debug)]
pub struct CreatureDefinition {
    pub name: &'static str,
    pub max_power: u64,
    /// Max base cooldown in seconds => will be influenced by config and stats
    pub breeding_cooldown: u64,
    // If the creature can be summoned => cooldown in seconds
    pub summoning_cooldown: Option<u64>,
    pub tier: CreatureTier,
    pub item_drops: &'static [CreatureItemDrop],
    pub sprite_png: &'static [u8],
}
