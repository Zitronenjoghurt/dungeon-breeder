use tier::CreatureTier;

pub mod tier;

#[derive(Debug)]
pub struct CreatureDefinition {
    pub name: &'static str,
    pub max_power: u64,
    pub tier: CreatureTier,
}
