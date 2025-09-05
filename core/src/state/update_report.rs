use crate::state::item::collection::ItemCollection;
use crate::state::item::NewItem;

#[derive(Debug, Default)]
pub struct GameStateUpdateReport {
    pub ticks_elapsed: u64,
    pub obtained_items: ItemCollection,
    pub times_specimen_slain: u64,
}

impl GameStateUpdateReport {
    pub fn on_items_obtained(&mut self, items: &[NewItem]) {
        self.obtained_items.add_new_batch(items);
    }

    pub fn on_specimen_slain(&mut self) {
        self.times_specimen_slain += 1;
    }

    pub fn on_tick(&mut self) {
        self.ticks_elapsed += 1;
    }
}
