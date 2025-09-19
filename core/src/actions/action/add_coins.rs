use crate::actions::action::GameActionHandler;
use crate::error::GameResult;
use crate::events::GameEvents;
use crate::state::GameState;

#[derive(Debug)]
pub struct AddCoinsAction {
    pub amount: u128,
}

impl GameActionHandler for AddCoinsAction {
    fn handle(self, state: &mut GameState, bus: &mut GameEvents) -> GameResult<()> {
        state.treasury.add_coins(self.amount);
        Ok(())
    }
}
