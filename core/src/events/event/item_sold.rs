use crate::data::item::id::ItemID;

#[derive(Debug)]
pub struct ItemSoldEvent {
    pub item_id: ItemID,
    pub amount: u64,
}
