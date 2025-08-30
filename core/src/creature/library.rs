use crate::creature::Creature;
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
}
