use crate::data::item::id::ItemID;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ItemCompendium {
    entries: HashMap<ItemID, ItemCompendiumEntry>,
}

impl ItemCompendium {
    pub fn on_item_obtained(&mut self, item_id: &ItemID, amount: u64) {
        self.entries
            .entry(*item_id)
            .or_default()
            .on_obtained(amount);
    }

    pub fn on_item_sold(&mut self, item_id: &ItemID, amount: u64) {
        self.entries.entry(*item_id).or_default().on_sold(amount);
    }

    pub fn has_unlocked(&self, item_id: &ItemID) -> bool {
        self.entries.contains_key(item_id)
    }

    pub fn iter_unlocked_ids(&self) -> impl Iterator<Item = &ItemID> {
        self.entries.keys()
    }

    pub fn iter_entries(&self) -> impl Iterator<Item = (&ItemID, &ItemCompendiumEntry)> {
        self.entries.iter()
    }

    pub fn unlocked_ids(&self) -> Vec<ItemID> {
        self.entries.keys().copied().collect()
    }

    pub fn get_entry(&self, item_id: &ItemID) -> Option<&ItemCompendiumEntry> {
        self.entries.get(item_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCompendiumEntry {
    pub unlocked_at: DateTime<Utc>,
    pub times_obtained: u128,
    pub times_sold: u128,
}

impl Default for ItemCompendiumEntry {
    fn default() -> Self {
        Self {
            unlocked_at: Utc::now(),
            times_obtained: 0,
            times_sold: 0,
        }
    }
}

impl ItemCompendiumEntry {
    pub fn on_obtained(&mut self, amount: u64) {
        self.times_obtained = self.times_obtained.saturating_add(amount as u128);
    }

    pub fn on_sold(&mut self, amount: u64) {
        self.times_sold = self.times_sold.saturating_add(amount as u128);
    }
}
