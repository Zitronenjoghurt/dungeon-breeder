use crate::state::dungeon::layer::slot::DungeonLayerSlot;
use crate::state::item::collection::ItemCollection;
use crate::state::specimen::collection::SpecimenCollection;
use crate::state::specimen::SpecimenId;
use crate::state::update_report::GameStateUpdateReport;
use crate::systems::upgrade_costs::dungeon_layer_slot_unlock_cost;
use serde::{Deserialize, Serialize};

pub mod slot;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DungeonLayer {
    slots: Vec<DungeonLayerSlot>,
}

impl DungeonLayer {
    pub fn tick(
        &mut self,
        report: &mut GameStateUpdateReport,
        specimen: &mut SpecimenCollection,
        items: &mut ItemCollection,
    ) {
        for slot in self.slots.iter_mut() {
            slot.tick(report, specimen, items)
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

    pub fn unlock_slot(&mut self) -> usize {
        let index = self.slots.len();
        self.slots.push(DungeonLayerSlot::default());
        index
    }

    pub fn next_slot_index(&self) -> usize {
        self.slots.len()
    }

    pub fn next_slot_costs(&self, layer_index: usize) -> Option<u128> {
        dungeon_layer_slot_unlock_cost(layer_index, self.next_slot_index())
    }

    pub fn has_slot_to_unlock(&self, layer_index: usize) -> bool {
        self.next_slot_costs(layer_index).is_some()
    }

    pub fn iter_slot_assigned_specimen(&self) -> impl Iterator<Item = SpecimenId> {
        self.slots
            .iter()
            .filter_map(|slot| slot.get_assigned_specimen_id())
    }
}
