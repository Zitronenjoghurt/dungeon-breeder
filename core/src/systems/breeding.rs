use crate::state::specimen::{NewSpecimen, Specimen};
use crate::utils::random::random_normal_combination_01;

pub fn breed_specimen(specimen_a: &Specimen, specimen_b: &Specimen) -> Option<NewSpecimen> {
    if specimen_a.creature_id != specimen_b.creature_id || specimen_a.id == specimen_b.id {
        return None;
    }

    let strength = random_normal_combination_01(specimen_a.strength, specimen_b.strength);
    let intelligence =
        random_normal_combination_01(specimen_a.intelligence, specimen_b.intelligence);
    let vitality = random_normal_combination_01(specimen_a.vitality, specimen_b.vitality);
    let agility = random_normal_combination_01(specimen_a.agility, specimen_b.agility);

    let new_specimen = NewSpecimen {
        creature_id: specimen_a.creature_id,
        strength,
        intelligence,
        vitality,
        agility,
    };

    Some(new_specimen)
}
