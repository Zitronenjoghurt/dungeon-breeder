use crate::creature::{Creature, CreatureId};
use crate::data::GameData;
use crate::utils::random::{
    random_normal, random_normal_combination_01, random_normal_exp_bias, random_normalized,
};
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

    pub fn get_creature<'a>(&self, data: &'a GameData) -> Option<&'a Creature> {
        data.creatures.get_by_id(self.creature_id)
    }

    pub fn breed_with(&self, other: &Specimen) -> Option<NewSpecimen> {
        if self.creature_id != other.creature_id || self.id == other.id {
            return None;
        }

        let strength = random_normal_combination_01(self.strength, other.strength);
        let intelligence = random_normal_combination_01(self.intelligence, other.intelligence);
        let vitality = random_normal_combination_01(self.vitality, other.vitality);
        let agility = random_normal_combination_01(self.agility, other.agility);

        let new_specimen = NewSpecimen {
            creature_id: self.creature_id,
            strength,
            intelligence,
            vitality,
            agility,
        };

        Some(new_specimen)
    }

    pub fn fuse_with(&self, other: &Specimen, data: &GameData) -> Option<NewSpecimen> {
        if self.id == other.id {
            return None;
        }

        let creature_a = self.get_creature(data)?;
        let creature_b = other.get_creature(data)?;

        let fusion_power = (self.proficiency() * creature_a.max_power as f32
            + other.proficiency() * creature_b.max_power as f32)
            * random_normal();

        let creature_candidates = data
            .creatures
            .get_fusion_candidates(fusion_power, &data.config);
        let index = (creature_candidates.len() as f32 * random_normalized()) as usize;
        let selected_creature = creature_candidates[index];

        let avg_max_power = (creature_a.max_power as f32 + creature_b.max_power as f32) / 2.0;
        let power_ratio = avg_max_power / selected_creature.max_power as f32;
        let strength_factor =
            power_ratio * random_normal_exp_bias(data.config.fusion_power_ratio_exp_bias);
        let intelligence_factor =
            power_ratio * random_normal_exp_bias(data.config.fusion_power_ratio_exp_bias);
        let vitality_factor =
            power_ratio * random_normal_exp_bias(data.config.fusion_power_ratio_exp_bias);
        let agility_factor =
            power_ratio * random_normal_exp_bias(data.config.fusion_power_ratio_exp_bias);

        let new_specimen = NewSpecimen {
            creature_id: selected_creature.id,
            strength: self.strength * strength_factor,
            intelligence: self.intelligence * intelligence_factor,
            vitality: self.vitality * vitality_factor,
            agility: self.agility * agility_factor,
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
            strength: random_normal(),
            intelligence: random_normal(),
            vitality: random_normal(),
            agility: random_normal(),
        }
    }
}
