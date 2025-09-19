use crate::actions::action::GameActionHandler;
use crate::error::GameResult;
use crate::events::GameEvents;
use crate::state::GameState;

#[derive(Debug)]
pub struct ResetGameStateAction;

impl GameActionHandler for ResetGameStateAction {
    fn handle(self, state: &mut GameState, events: &mut GameEvents) -> GameResult<()> {
        *state = GameState::default();
        Ok(())
    }
}
