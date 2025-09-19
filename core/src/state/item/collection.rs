use crate::data::creature::id::CreatureID;
use crate::data::item::id::ItemID;
use crate::events::event::GameEvent;
use crate::events::GameEvents;
use crate::state::item::compendium::ItemCompendium;
use crate::state::item::NewItem;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ItemCollection {
    collection: BTreeMap<ItemID, u64>,
    compendium: ItemCompendium,
}

impl ItemCollection {
    pub fn compendium(&self) -> &ItemCompendium {
        &self.compendium
    }

    pub fn add_new(&mut self, bus: &mut GameEvents, new_item: &NewItem) {
        let current_amount = self.collection.entry(new_item.item_id).or_insert(0);
        *current_amount = current_amount.saturating_add(new_item.amount);
        bus.item_obtained(new_item.item_id, new_item.amount);
    }

    pub fn add_new_batch(&mut self, bus: &mut GameEvents, new_items: &[NewItem]) {
        new_items
            .iter()
            .for_each(|new_item| self.add_new(bus, new_item));
    }

    pub fn remove_item(&mut self, item_id: ItemID, amount: u64) -> bool {
        let current_amount = self.collection.entry(item_id).or_insert(0);
        if *current_amount < amount {
            return false;
        }

        *current_amount = current_amount.saturating_sub(amount);
        if *current_amount == 0 {
            self.collection.remove(&item_id);
        }

        true
    }

    pub fn get_count(&self, item_id: ItemID) -> u64 {
        self.collection.get(&item_id).copied().unwrap_or_default()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ItemID, &u64)> {
        self.collection.iter()
    }

    pub fn len(&self) -> usize {
        self.collection.len()
    }

    pub fn is_empty(&self) -> bool {
        self.collection.is_empty()
    }
}

// Events
impl ItemCollection {
    pub fn handle_event(&mut self, bus: &mut GameEvents, event: &GameEvent) {
        match event {
            GameEvent::ItemObtained(event) => self.on_item_obtained(event.item_id, event.amount),
            GameEvent::ItemSold(event) => self.on_item_sold(event.item_id, event.amount),
            GameEvent::SpecimenSlain(event) => {
                self.on_specimen_slain(bus, event.creature_id, event.proficiency)
            }
            _ => {}
        }
    }

    pub fn on_item_obtained(&mut self, item_id: ItemID, amount: u64) {
        self.compendium.on_item_obtained(&item_id, amount);
    }

    pub fn on_specimen_slain(
        &mut self,
        bus: &mut GameEvents,
        creature_id: CreatureID,
        proficiency: f32,
    ) {
        let dropped_items = creature_id.generate_drops(proficiency);
        self.add_new_batch(bus, &dropped_items);
    }

    pub fn on_item_sold(&mut self, item_id: ItemID, amount: u64) {
        self.compendium.on_item_sold(&item_id, amount);
    }
}
