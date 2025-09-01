use crate::data::config::CONFIG;
use crate::data::creature::def::CreatureDefinition;
use crate::data::creature::id::CreatureID;
use crate::state::item::NewItem;
use crate::utils::random::random_normal;
use serde::{Deserialize, Serialize};

pub mod collection;

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

    pub fn power(&self) -> f32 {
        self.proficiency() * self.creature_def().max_power as f32
    }

    pub fn slay_duration_secs(&self) -> u64 {
        self.power().powf(CONFIG.slay_duration_power_exponent) as u64
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

    pub fn generate_drops(&self) -> Vec<NewItem> {
        let mut rng = rand::rng();
        self.creature_id
            .def()
            .item_drops
            .iter()
            .filter_map(|item_drop| {
                let count = item_drop.generate_drop(&mut rng, self.proficiency())?;
                Some(NewItem {
                    item_id: item_drop.item_id,
                    amount: count as u64,
                })
            })
            .collect::<Vec<_>>()
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
