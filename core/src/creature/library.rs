use crate::creature::Creature;
use crate::data::config::Config;
use data::CREATURE_DATA;
use std::collections::HashMap;

mod data;

#[derive(Debug)]
pub struct CreatureLibrary {
    id_to_index: HashMap<u16, usize>,
}

impl Default for CreatureLibrary {
    fn default() -> Self {
        let mut id_to_index = HashMap::new();
        for (index, creature) in CREATURE_DATA.iter().enumerate() {
            id_to_index.insert(creature.id, index);
        }

        Self { id_to_index }
    }
}

impl CreatureLibrary {
    pub fn get_by_id(&self, id: u16) -> Option<&Creature> {
        self.id_to_index
            .get(&id)
            .map(|index| &CREATURE_DATA[*index])
    }

    pub fn get_fusion_candidates(&self, fusion_power: f32, config: &Config) -> Vec<&Creature> {
        let min_power = fusion_power * config.fusion_candidates_min_power_factor;
        let max_power = fusion_power * config.fusion_candidates_max_power_factor;

        let candidates = CREATURE_DATA
            .iter()
            .filter(|creature| (min_power..=max_power).contains(&(creature.max_power as f32)))
            .collect::<Vec<_>>();

        if candidates.is_empty() {
            vec![&CREATURE_DATA[0]]
        } else {
            candidates
        }
    }
}
