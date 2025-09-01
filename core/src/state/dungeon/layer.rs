use crate::state::dungeon::layer::slot::DungeonLayerSlot;
use crate::state::item::collection::ItemCollection;
use crate::state::specimen::collection::SpecimenCollection;
use serde::{Deserialize, Serialize};

pub mod slot;

#[derive(Debug, Serialize, Deserialize)]
pub struct DungeonLayer {
    slots: Vec<DungeonLayerSlot>,
}

impl Default for DungeonLayer {
    fn default() -> Self {
        Self {
            slots: vec![DungeonLayerSlot::default()],
        }
    }
}

impl DungeonLayer {
    pub fn tick(&mut self, specimen: &mut SpecimenCollection, items: &mut ItemCollection) {
        for slot in self.slots.iter_mut() {
            slot.tick(specimen, items)
        }
    }

    pub fn slot_count(&self) -> usize {
        self.slots.len()
    }

    pub fn get_slot(&self, index: usize) -> Option<&DungeonLayerSlot> {
        self.slots.get(index)
    }

    pub fn get_slot_mut(&mut self, index: usize) -> Option<&mut DungeonLayerSlot> {
        self.slots.get_mut(index)
    }

    pub fn iter_slots(&self) -> impl Iterator<Item = &DungeonLayerSlot> {
        self.slots.iter()
    }

    pub fn iter_slots_mut(&mut self) -> impl Iterator<Item = &mut DungeonLayerSlot> {
        self.slots.iter_mut()
    }

    pub fn add_slot(&mut self) {
        self.slots.push(DungeonLayerSlot::default());
    }
}
