use crate::error::{GameError, GameResult};
use crate::state::specimen::collection::SpecimenCollection;
use crate::state::specimen::{NewSpecimen, SpecimenId};
use crate::utils::random::random_normal_combination_01;
use std::cmp::max;

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

    if specimen_a.creature_id != specimen_b.creature_id {
        return Err(GameError::BreedingImpossibleIncompatibleCreatures);
    }

    if specimen_a.id == specimen_b.id {
        return Err(GameError::BreedingImpossibleSameSpecimen);
    }

    if !specimen_a.can_breed() || !specimen_b.can_breed() {
        return Err(GameError::BreedingImpossibleSpecimenOnCooldown);
    }

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

    if let Some(specimen_a_mut) = collection.get_by_id_mut(specimen_a_id) {
        specimen_a_mut.on_bred();
    }

    if let Some(specimen_b_mut) = collection.get_by_id_mut(specimen_b_id) {
        specimen_b_mut.on_bred();
    }

    Ok(new_specimen)
}
