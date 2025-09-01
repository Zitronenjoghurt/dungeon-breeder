use crate::state::dungeon::layer::slot::DungeonLayerSlot;
use crate::state::item::collection::ItemCollection;
use crate::state::specimen::collection::SpecimenCollection;
use serde::{Deserialize, Serialize};

pub mod slot;

#[derive(Debug, Serialize, Deserialize)]
pub struct DungeonLayer {
    slots: Vec<DungeonLayerSlot>,
}

impl DungeonLayer {
    pub fn tick(&mut self, specimen: &mut SpecimenCollection, items: &mut ItemCollection) {
        for slot in self.slots.iter_mut() {
            slot.tick(specimen, items)
        }
    }
}
