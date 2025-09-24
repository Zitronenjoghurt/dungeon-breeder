use crate::actions::action::GameActionHandler;
use crate::error::GameResult;
use crate::events::GameEvents;
use crate::state::GameState;
use crate::types::flag::GameFlag;

#[derive(Debug)]
pub struct SetFlagAction {
    pub flag: GameFlag,
    pub value: bool,
}

impl GameActionHandler for SetFlagAction {
    fn handle(self, state: &mut GameState, _bus: &mut GameEvents) -> GameResult<()> {
        state.flags.set_value(self.flag, self.value);
        Ok(())
    }
}
