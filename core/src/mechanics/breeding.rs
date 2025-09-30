use crate::data::config::CONFIG;
use crate::error::{GameError, GameResult};
use crate::state::specimen::obtain_method::SpecimenObtainMethod;
use crate::state::specimen::{NewSpecimen, Specimen};
use crate::utils::random::random_normal_combination_01;
use std::cmp::max;

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

pub fn breed_specimen(specimen_a: &Specimen, specimen_b: &Specimen) -> GameResult<NewSpecimen> {
    check_specimen_can_breed(specimen_a, specimen_b)?;

    let strength = breed_stats(specimen_a.strength, specimen_b.strength);
    let intelligence = breed_stats(specimen_a.intelligence, specimen_b.intelligence);
    let vitality = breed_stats(specimen_a.vitality, specimen_b.vitality);
    let agility = breed_stats(specimen_a.agility, specimen_b.agility);
    let regeneration = breed_stats(specimen_a.regeneration, specimen_b.regeneration);
    let fertility = breed_stats(specimen_a.fertility, specimen_b.fertility);

    let new_specimen = NewSpecimen {
        creature_id: specimen_a.creature_id,
        obtain_method: SpecimenObtainMethod::Breeding,
        nickname: None,
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

    Ok(new_specimen)
}

fn breed_stats(a: f32, b: f32) -> f32 {
    let avg = (a + b) / 2.0;
    let bias = avg * (CONFIG.breeding_base_bias_factor - 1.0) + 1.0;

    if a == 1.0 && b == 1.0 {
        1.0
    } else {
        random_normal_combination_01(a, b, bias)
    }
}
