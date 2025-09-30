use crate::types::specimen_stat::SpecimenStat;
use crate::types::trend::Trend;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct BreedingStatTrends {
    pub proficiency: Trend,
    pub strength: Trend,
    pub intelligence: Trend,
    pub agility: Trend,
    pub vitality: Trend,
    pub regeneration: Trend,
    pub fertility: Trend,
}

impl BreedingStatTrends {
    pub fn get_stat(&self, stat: &SpecimenStat) -> Trend {
        match stat {
            SpecimenStat::Proficiency => self.proficiency,
            SpecimenStat::Strength => self.strength,
            SpecimenStat::Intelligence => self.intelligence,
            SpecimenStat::Agility => self.agility,
            SpecimenStat::Vitality => self.vitality,
            SpecimenStat::Regeneration => self.regeneration,
            SpecimenStat::Fertility => self.fertility,
        }
    }

    pub fn set_stat(&mut self, stat: &SpecimenStat, trend: Trend) {
        match stat {
            SpecimenStat::Proficiency => self.proficiency = trend,
            SpecimenStat::Strength => self.strength = trend,
            SpecimenStat::Intelligence => self.intelligence = trend,
            SpecimenStat::Agility => self.agility = trend,
            SpecimenStat::Vitality => self.vitality = trend,
            SpecimenStat::Regeneration => self.regeneration = trend,
            SpecimenStat::Fertility => self.fertility = trend,
        }
    }
}
