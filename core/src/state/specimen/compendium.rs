use crate::data::creature::id::CreatureID;
use crate::state::specimen::Specimen;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreatureCompendium {
    entries: HashMap<CreatureID, CreatureCompendiumEntry>,
}

impl CreatureCompendium {
    pub fn on_specimen_obtained(&mut self, specimen: &Specimen) {
        self.entries
            .entry(specimen.creature_id)
            .or_default()
            .update(specimen);
    }

    pub fn on_specimen_slain(&mut self, creature_id: &CreatureID) {
        self.entries.entry(*creature_id).or_default().times_slain += 1;
    }

    pub fn on_specimen_bred(&mut self, creature_id: &CreatureID) {
        self.entries.entry(*creature_id).or_default().times_bred += 1;
    }

    pub fn on_specimen_fused(&mut self, creature_id: &CreatureID) {
        self.entries.entry(*creature_id).or_default().times_fused += 1;
    }

    pub fn has_unlocked(&self, creature_id: &CreatureID) -> bool {
        self.entries.contains_key(creature_id)
    }

    pub fn iter_unlocked_ids(&self) -> impl Iterator<Item = &CreatureID> {
        self.entries.keys()
    }

    pub fn iter_entries(&self) -> impl Iterator<Item = (&CreatureID, &CreatureCompendiumEntry)> {
        self.entries.iter()
    }

    pub fn unlocked_ids(&self) -> Vec<CreatureID> {
        self.entries.keys().copied().collect()
    }

    pub fn get_entry(&self, creature_id: &CreatureID) -> Option<&CreatureCompendiumEntry> {
        self.entries.get(creature_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatureCompendiumEntry {
    pub unlocked_at: DateTime<Utc>,
    pub times_slain: u128,
    pub times_bred: u128,
    pub times_fused: u128,
    pub proficiency: (f32, f32),
    pub strength: (f32, f32),
    pub intelligence: (f32, f32),
    pub vitality: (f32, f32),
    pub agility: (f32, f32),
    pub regeneration: (f32, f32),
    pub fertility: (f32, f32),
}

impl Default for CreatureCompendiumEntry {
    fn default() -> Self {
        Self {
            unlocked_at: Utc::now(),
            times_slain: 0,
            times_bred: 0,
            times_fused: 0,
            proficiency: (1.0, 0.0),
            strength: (1.0, 0.0),
            intelligence: (1.0, 0.0),
            vitality: (1.0, 0.0),
            agility: (1.0, 0.0),
            regeneration: (1.0, 0.0),
            fertility: (1.0, 0.0),
        }
    }
}

impl CreatureCompendiumEntry {
    pub fn update(&mut self, specimen: &Specimen) {
        if specimen.proficiency() < self.proficiency.0 {
            self.proficiency.0 = specimen.proficiency();
        }
        if specimen.proficiency() > self.proficiency.1 {
            self.proficiency.1 = specimen.proficiency();
        }

        if specimen.strength < self.strength.0 {
            self.strength.0 = specimen.strength;
        }
        if specimen.strength > self.strength.1 {
            self.strength.1 = specimen.strength;
        }

        if specimen.intelligence < self.intelligence.0 {
            self.intelligence.0 = specimen.intelligence;
        }
        if specimen.intelligence > self.intelligence.1 {
            self.intelligence.1 = specimen.intelligence;
        }

        if specimen.vitality < self.vitality.0 {
            self.vitality.0 = specimen.vitality;
        }
        if specimen.vitality > self.vitality.1 {
            self.vitality.1 = specimen.vitality;
        }

        if specimen.agility < self.agility.0 {
            self.agility.0 = specimen.agility;
        }
        if specimen.agility > self.agility.1 {
            self.agility.1 = specimen.agility;
        }

        if specimen.regeneration < self.regeneration.0 {
            self.regeneration.0 = specimen.regeneration;
        }
        if specimen.regeneration > self.regeneration.1 {
            self.regeneration.1 = specimen.regeneration;
        }

        if specimen.fertility < self.fertility.0 {
            self.fertility.0 = specimen.fertility;
        }
        if specimen.fertility > self.fertility.1 {
            self.fertility.1 = specimen.fertility;
        }
    }
}
