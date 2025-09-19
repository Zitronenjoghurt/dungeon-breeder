use crate::data::item::id::ItemID;

#[derive(Debug)]
pub struct ItemObtainedEvent {
    pub item_id: ItemID,
    pub amount: u64,
}
