use crate::data::item::id::ItemID;

pub mod collection;
pub mod compendium;

pub struct NewItem {
    pub item_id: ItemID,
    pub amount: u64,
}
