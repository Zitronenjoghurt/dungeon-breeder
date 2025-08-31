use crate::data::item::id::ItemID;

pub mod collection;

pub struct NewItem {
    pub item_id: ItemID,
    pub amount: u64,
}
