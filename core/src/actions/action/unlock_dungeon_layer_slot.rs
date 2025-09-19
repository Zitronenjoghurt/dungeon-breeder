use crate::actions::action::GameActionHandler;
use crate::error::{GameError, GameResult};
use crate::events::GameEvents;
use crate::state::GameState;

#[derive(Debug)]
pub struct UnlockDungeonLayerSlotAction {
    pub layer_index: usize,
}

impl GameActionHandler for UnlockDungeonLayerSlotAction {
    fn handle(self, state: &mut GameState, bus: &mut GameEvents) -> GameResult<()> {
        let Some(layer) = state.dungeon.get_layer_mut(self.layer_index) else {
            return Err(GameError::DungeonLayerNotFound(self.layer_index));
        };

        let Some(costs) = layer.next_slot_costs(self.layer_index) else {
            return Err(GameError::InsufficientCoins);
        };

        let success = state.treasury.remove_coins(costs);
        if success {
            let slot_index = layer.unlock_slot();
            Ok(())
        } else {
            Err(GameError::InsufficientCoins)
        }
    }
}
