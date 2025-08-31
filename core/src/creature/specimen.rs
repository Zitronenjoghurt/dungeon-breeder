use crate::creature::def::CreatureDefinition;
use crate::creature::id::CreatureID;
use crate::utils::random::*;
use serde::{Deserialize, Serialize};

pub type SpecimenId = u32;

#[derive(Debug, Serialize, Deserialize)]
pub struct Specimen {
    pub id: SpecimenId,
    pub creature_id: CreatureID,
    pub strength: f32,
    pub intelligence: f32,
    pub vitality: f32,
    pub agility: f32,
}

impl Specimen {
    pub fn creature_def(&self) -> &'static CreatureDefinition {
        self.creature_id.def()
    }

    pub fn proficiency(&self) -> f32 {
        (self.strength + self.intelligence + self.vitality + self.agility) / 4.0
    }

    pub fn power(&self, max_power: u64) -> f32 {
        self.proficiency() * max_power as f32
    }

    pub fn from_new_specimen(id: SpecimenId, new_specimen: NewSpecimen) -> Specimen {
        Specimen {
            id,
            creature_id: new_specimen.creature_id,
            strength: new_specimen.strength,
            intelligence: new_specimen.intelligence,
            vitality: new_specimen.vitality,
            agility: new_specimen.agility,
        }
    }
}

#[derive(Debug)]
pub struct NewSpecimen {
    pub creature_id: CreatureID,
    pub strength: f32,
    pub intelligence: f32,
    pub vitality: f32,
    pub agility: f32,
}

impl NewSpecimen {
    pub fn random_from_creature_id(creature_id: CreatureID) -> NewSpecimen {
        NewSpecimen {
            creature_id,
            strength: random_normal(),
            intelligence: random_normal(),
            vitality: random_normal(),
            agility: random_normal(),
        }
    }
}
