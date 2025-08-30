use crate::creature::tier::CreatureTier;

pub mod library;
pub mod specimen;
mod tier;

#[derive(Debug)]
pub struct Creature {
    pub id: u16,
    pub name: &'static str,
    pub max_power: u64,
    pub tier: CreatureTier,
}
