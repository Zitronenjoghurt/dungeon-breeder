use crate::data::item::id::ItemID;
use crate::state::item::NewItem;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ItemCollection {
    collection: BTreeMap<ItemID, u64>,
}

impl ItemCollection {
    pub fn add_new(&mut self, new_item: &NewItem) {
        let current_amount = self.collection.entry(new_item.item_id).or_insert(0);
        *current_amount = current_amount.saturating_add(new_item.amount);
    }

    pub fn add_new_batch(&mut self, new_items: &[NewItem]) {
        new_items.iter().for_each(|new_item| self.add_new(new_item));
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
