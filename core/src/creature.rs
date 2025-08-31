use crate::creature::tier::CreatureTier;

pub mod library;
pub mod specimen;
mod tier;

pub type CreatureId = u16;

#[derive(Debug)]
pub struct Creature {
    pub id: CreatureId,
    pub name: &'static str,
    pub max_power: u64,
    pub tier: CreatureTier,
}
