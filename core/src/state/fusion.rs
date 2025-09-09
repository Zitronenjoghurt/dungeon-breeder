use crate::data::config::CONFIG;
use crate::data::creature::id::CreatureID;
use crate::error::{GameError, GameResult};
use crate::state::specimen::obtain_method::SpecimenObtainMethod;
use crate::state::specimen::{NewSpecimen, Specimen, SpecimenId};
use crate::utils::random::{random_normal, random_normal_exp_bias, random_normalized};
use serde::{Deserialize, Serialize};
use std::cmp::max;
use strum::IntoEnumIterator;

pub mod simulation;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FusionState {
    last_fusion_result_id: Option<SpecimenId>,
}

impl FusionState {
    pub fn on_successful_fusion(&mut self, new_specimen_id: SpecimenId) {
        self.last_fusion_result_id = Some(new_specimen_id);
    }

    pub fn last_fusion_result_id(&self) -> Option<SpecimenId> {
        self.last_fusion_result_id
    }
}

pub fn check_specimen_can_fuse(specimen_1: &Specimen, specimen_2: &Specimen) -> GameResult<()> {
    if specimen_1.id == specimen_2.id {
        Err(GameError::FusionImpossibleSameSpecimen)
    } else {
        Ok(())
    }
}

pub fn generate_fusion_power(specimen_a: &Specimen, specimen_b: &Specimen) -> f32 {
    (specimen_a.power() + specimen_b.power()) * random_normal()
}

pub fn determine_fusion_candidates(fusion_power: f32) -> Vec<CreatureID> {
    let min_power = fusion_power * CONFIG.fusion_candidates_min_power_factor;
    let max_power = fusion_power * CONFIG.fusion_candidates_max_power_factor;

    let candidates: Vec<CreatureID> = CreatureID::iter()
        .filter(|id| {
            let power = id.def().max_power as f32;
            min_power <= power && power <= max_power
        })
        .collect();

    if candidates.is_empty() {
        vec![CreatureID::Gonk]
    } else {
        candidates
    }
}

pub fn fuse_specimen(specimen_a: &Specimen, specimen_b: &Specimen) -> GameResult<NewSpecimen> {
    check_specimen_can_fuse(specimen_a, specimen_b)?;

    let fusion_power = generate_fusion_power(specimen_a, specimen_b);
    let creature_candidates = determine_fusion_candidates(fusion_power);
    let index = ((creature_candidates.len() as f32 * random_normalized()) as usize)
        .min(creature_candidates.len() - 1);
    let selected_creature_id = creature_candidates[index];
    let selected_creature = selected_creature_id.def();

    let avg_max_power = (specimen_a.creature_def().max_power as f32
        + specimen_b.creature_def().max_power as f32)
        / 2.0;
    let power_ratio = avg_max_power / selected_creature.max_power as f32;

    let strength_factor = power_ratio * random_normal_exp_bias(CONFIG.fusion_power_ratio_exp_bias);
    let intelligence_factor =
        power_ratio * random_normal_exp_bias(CONFIG.fusion_power_ratio_exp_bias);
    let vitality_factor = power_ratio * random_normal_exp_bias(CONFIG.fusion_power_ratio_exp_bias);
    let agility_factor = power_ratio * random_normal_exp_bias(CONFIG.fusion_power_ratio_exp_bias);
    let regeneration_factor =
        power_ratio * random_normal_exp_bias(CONFIG.fusion_power_ratio_exp_bias);
    let fertility_factor = power_ratio * random_normal_exp_bias(CONFIG.fusion_power_ratio_exp_bias);

    let avg_strength = (specimen_a.strength + specimen_b.strength) / 2.0;
    let avg_intelligence = (specimen_a.intelligence + specimen_b.intelligence) / 2.0;
    let avg_vitality = (specimen_a.vitality + specimen_b.vitality) / 2.0;
    let avg_agility = (specimen_a.agility + specimen_b.agility) / 2.0;
    let avg_regeneration = (specimen_a.regeneration + specimen_b.regeneration) / 2.0;
    let avg_fertility = (specimen_a.fertility + specimen_b.fertility) / 2.0;

    let new_specimen = NewSpecimen {
        creature_id: selected_creature_id,
        obtain_method: SpecimenObtainMethod::Fusion,
        strength: (avg_strength * strength_factor).clamp(0.0, 1.0),
        intelligence: (avg_intelligence * intelligence_factor).clamp(0.0, 1.0),
        vitality: (avg_vitality * vitality_factor).clamp(0.0, 1.0),
        agility: (avg_agility * agility_factor).clamp(0.0, 1.0),
        regeneration: (avg_regeneration * regeneration_factor).clamp(0.0, 1.0),
        fertility: (avg_fertility * fertility_factor).clamp(0.0, 1.0),
        breeding_generation: max(
            specimen_a.breeding_generation,
            specimen_b.breeding_generation,
        ),
        fusion_generation: max(specimen_a.fusion_generation, specimen_b.fusion_generation) + 1,
    };

    Ok(new_specimen)
}
