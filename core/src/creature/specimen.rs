use crate::creature::CreatureId;
use crate::utils::random::{random_normalized, random_normalized_fusion};
use serde::{Deserialize, Serialize};

pub type SpecimenId = u32;

#[derive(Debug, Serialize, Deserialize)]
pub struct Specimen {
    pub id: SpecimenId,
    pub creature_id: CreatureId,
    pub strength: f32,
    pub intelligence: f32,
    pub vitality: f32,
    pub agility: f32,
}

impl Specimen {
    pub fn power(&self) -> f32 {
        (self.strength + self.intelligence + self.vitality + self.agility) / 4.0
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

    pub fn breed_with(&self, other: &Specimen) -> Option<NewSpecimen> {
        if self.creature_id != other.creature_id || self.id == other.id {
            return None;
        }

        let strength = random_normalized_fusion(self.strength, other.strength);
        let intelligence = random_normalized_fusion(self.intelligence, other.intelligence);
        let vitality = random_normalized_fusion(self.vitality, other.vitality);
        let agility = random_normalized_fusion(self.agility, other.agility);

        let new_specimen = NewSpecimen {
            creature_id: self.creature_id,
            strength,
            intelligence,
            vitality,
            agility,
        };

        Some(new_specimen)
    }
}

#[derive(Debug)]
pub struct NewSpecimen {
    pub creature_id: CreatureId,
    pub strength: f32,
    pub intelligence: f32,
    pub vitality: f32,
    pub agility: f32,
}

impl NewSpecimen {
    pub fn random_from_creature_id(creature_id: CreatureId) -> NewSpecimen {
        NewSpecimen {
            creature_id,
            strength: random_normalized(),
            intelligence: random_normalized(),
            vitality: random_normalized(),
            agility: random_normalized(),
        }
    }
}
