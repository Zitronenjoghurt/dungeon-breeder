use crate::data::item::id::ItemID;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemObtainedEvent {
    pub item_id: ItemID,
    pub amount: u64,
}
