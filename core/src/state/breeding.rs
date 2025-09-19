use crate::error::{GameError, GameResult};
use crate::state::specimen::collection::SpecimenCollection;
use crate::state::specimen::obtain_method::SpecimenObtainMethod;
use crate::state::specimen::{NewSpecimen, Specimen, SpecimenId};
use crate::utils::random::random_normal_combination_01;
use serde::{Deserialize, Serialize};
use std::cmp::max;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BreedingState {
    last_parent_id_1: Option<SpecimenId>,
    last_parent_id_2: Option<SpecimenId>,
    last_offspring_id: Option<SpecimenId>,
}

impl BreedingState {
    pub fn last_parent_id_1(&self) -> Option<SpecimenId> {
        self.last_parent_id_1
    }

    pub fn last_parent_id_2(&self) -> Option<SpecimenId> {
        self.last_parent_id_2
    }

    pub fn last_offspring_id(&self) -> Option<SpecimenId> {
        self.last_offspring_id
    }

    pub fn on_successful_breed(
        &mut self,
        parent_1_id: SpecimenId,
        parent_2_id: SpecimenId,
        new_specimen_id: SpecimenId,
    ) {
        self.last_parent_id_1 = Some(parent_1_id);
        self.last_parent_id_2 = Some(parent_2_id);
        self.last_offspring_id = Some(new_specimen_id);
    }
}

pub fn check_specimen_can_breed(specimen_1: &Specimen, specimen_2: &Specimen) -> GameResult<()> {
    if specimen_1.creature_id != specimen_2.creature_id {
        return Err(GameError::BreedingImpossibleIncompatibleCreatures);
    }

    if specimen_1.id == specimen_2.id {
        return Err(GameError::BreedingImpossibleSameSpecimen);
    }

    if !specimen_1.can_breed() || !specimen_2.can_breed() {
        return Err(GameError::BreedingImpossibleSpecimenOnCooldown);
    }

    Ok(())
}

pub fn breed_specimen(
    collection: &mut SpecimenCollection,
    specimen_a_id: SpecimenId,
    specimen_b_id: SpecimenId,
) -> GameResult<NewSpecimen> {
    let Some(specimen_a) = collection.get_by_id(specimen_a_id) else {
        return Err(GameError::SpecimenNotFound(specimen_a_id));
    };

    let Some(specimen_b) = collection.get_by_id(specimen_b_id) else {
        return Err(GameError::SpecimenNotFound(specimen_b_id));
    };

    let creature_a_id = specimen_a.creature_id;
    let creature_b_id = specimen_b.creature_id;

    check_specimen_can_breed(specimen_a, specimen_b)?;

    let strength = random_normal_combination_01(specimen_a.strength, specimen_b.strength);
    let intelligence =
        random_normal_combination_01(specimen_a.intelligence, specimen_b.intelligence);
    let vitality = random_normal_combination_01(specimen_a.vitality, specimen_b.vitality);
    let agility = random_normal_combination_01(specimen_a.agility, specimen_b.agility);
    let regeneration =
        random_normal_combination_01(specimen_a.regeneration, specimen_b.regeneration);
    let fertility = random_normal_combination_01(specimen_a.fertility, specimen_b.fertility);

    let new_specimen = NewSpecimen {
        creature_id: specimen_a.creature_id,
        obtain_method: SpecimenObtainMethod::Breeding,
        strength,
        intelligence,
        vitality,
        agility,
        regeneration,
        fertility,
        breeding_generation: max(
            specimen_a.breeding_generation,
            specimen_b.breeding_generation,
        ) + 1,
        fusion_generation: max(specimen_a.fusion_generation, specimen_b.fusion_generation),
    };

    collection.on_specimen_bred(&creature_a_id);
    collection.on_specimen_bred(&creature_b_id);

    if let Some(specimen_a_mut) = collection.get_by_id_mut(specimen_a_id) {
        specimen_a_mut.on_bred();
    }

    if let Some(specimen_b_mut) = collection.get_by_id_mut(specimen_b_id) {
        specimen_b_mut.on_bred();
    }

    Ok(new_specimen)
}
