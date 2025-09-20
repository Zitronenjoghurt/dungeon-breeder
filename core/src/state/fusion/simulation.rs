use crate::data::creature::id::CreatureID;
use crate::error::GameResult;
use crate::mechanics::fusion::fuse_specimen;
use crate::state::specimen::Specimen;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Default)]
pub struct FusionSimulation {
    creature_probabilities: BTreeMap<CreatureID, f32>,
}

impl FusionSimulation {
    #[tracing::instrument(
        target = "game",
        name = "game::state::fusion::simulation::simulate",
        level = "trace",
        skip(specimen_1, specimen_2)
    )]
    pub fn simulate(
        specimen_1: &Specimen,
        specimen_2: &Specimen,
        samples: usize,
    ) -> GameResult<Self> {
        let mut creature_counts: HashMap<CreatureID, usize> = HashMap::new();

        for _ in 0..samples {
            let new_specimen = fuse_specimen(specimen_1, specimen_2)?;
            let creature_id = new_specimen.creature_id;
            creature_counts
                .entry(creature_id)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let creature_probabilities = creature_counts
            .iter()
            .map(|(id, count)| (*id, *count as f32 / samples as f32))
            .collect();

        Ok(Self {
            creature_probabilities,
        })
    }

    pub fn iter_creature_probabilities(&self) -> impl Iterator<Item = (&CreatureID, &f32)> {
        self.creature_probabilities.iter()
    }
}
