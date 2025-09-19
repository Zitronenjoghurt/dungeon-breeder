use crate::actions::action::GameActionHandler;
use crate::data::item::id::ItemID;
use crate::error::{GameError, GameResult};
use crate::events::GameEvents;
use crate::state::GameState;

#[derive(Debug)]
pub struct SellItemAction {
    pub item_id: ItemID,
    pub amount: u64,
}

impl GameActionHandler for SellItemAction {
    fn handle(self, state: &mut GameState, events: &mut GameEvents) -> GameResult<()> {
        let success = state.items.remove_item(self.item_id, self.amount);
        if success {
            let coins = self.item_id.def().price as u128 * self.amount as u128;
            state.treasury.add_coins(coins);
            state.items.on_item_sold(&self.item_id, self.amount);
            Ok(())
        } else {
            Err(GameError::InsufficientItems)
        }
    }
}
