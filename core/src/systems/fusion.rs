use crate::data::config::CONFIG;
use crate::data::creature::id::CreatureID;
use crate::state::specimen::{NewSpecimen, Specimen};
use crate::utils::random::{random_normal, random_normal_exp_bias, random_normalized};
use strum::IntoEnumIterator;

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

pub fn fuse_specimen(specimen_a: &Specimen, specimen_b: &Specimen) -> Option<NewSpecimen> {
    if specimen_a.id == specimen_b.id {
        return None;
    }

    let creature_a = specimen_a.creature_def();
    let creature_b = specimen_b.creature_def();

    let fusion_power = (specimen_a.proficiency() * creature_a.max_power as f32
        + specimen_b.proficiency() * creature_b.max_power as f32)
        * random_normal();

    let creature_candidates = determine_fusion_candidates(fusion_power);
    let index = (creature_candidates.len() as f32 * random_normalized()) as usize;
    let selected_creature_id = creature_candidates[index];
    let selected_creature = selected_creature_id.def();

    let avg_max_power = (creature_a.max_power as f32 + creature_b.max_power as f32) / 2.0;
    let power_ratio = avg_max_power / selected_creature.max_power as f32;

    let strength_factor = power_ratio * random_normal_exp_bias(CONFIG.fusion_power_ratio_exp_bias);
    let intelligence_factor =
        power_ratio * random_normal_exp_bias(CONFIG.fusion_power_ratio_exp_bias);
    let vitality_factor = power_ratio * random_normal_exp_bias(CONFIG.fusion_power_ratio_exp_bias);
    let agility_factor = power_ratio * random_normal_exp_bias(CONFIG.fusion_power_ratio_exp_bias);

    let avg_strength = (specimen_a.strength + specimen_b.strength) / 2.0;
    let avg_intelligence = (specimen_a.intelligence + specimen_b.intelligence) / 2.0;
    let avg_vitality = (specimen_a.vitality + specimen_b.vitality) / 2.0;
    let avg_agility = (specimen_a.agility + specimen_b.agility) / 2.0;

    let new_specimen = NewSpecimen {
        creature_id: selected_creature_id,
        strength: avg_strength * strength_factor,
        intelligence: avg_intelligence * intelligence_factor,
        vitality: avg_vitality * vitality_factor,
        agility: avg_agility * agility_factor,
    };

    Some(new_specimen)
}
