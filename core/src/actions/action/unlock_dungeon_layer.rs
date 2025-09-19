use crate::actions::action::GameActionHandler;
use crate::error::{GameError, GameResult};
use crate::events::GameEvents;
use crate::state::GameState;

#[derive(Debug)]
pub struct UnlockDungeonLayerAction;

impl GameActionHandler for UnlockDungeonLayerAction {
    fn handle(self, state: &mut GameState, events: &mut GameEvents) -> GameResult<()> {
        let Some(costs) = state.dungeon.next_layer_costs() else {
            return Err(GameError::InsufficientCoins);
        };

        let success = state.treasury.remove_coins(costs);
        if success {
            let layer_index = state.dungeon.unlock_layer();
            Ok(())
        } else {
            Err(GameError::InsufficientCoins)
        }
    }
}
