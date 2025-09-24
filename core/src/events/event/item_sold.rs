use crate::data::item::id::ItemID;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemSoldEvent {
    pub item_id: ItemID,
    pub amount: u64,
}
