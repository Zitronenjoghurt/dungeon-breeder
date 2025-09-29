use crate::actions::action::GameActionHandler;
use crate::error::GameResult;
use crate::events::GameEvents;
use crate::state::GameState;

#[derive(Debug)]
pub struct DebugSkipTimeAction {
    pub seconds: u64,
}

impl GameActionHandler for DebugSkipTimeAction {
    fn handle(self, _state: &mut GameState, bus: &mut GameEvents) -> GameResult<()> {
        bus.do_skip_time(self.seconds);
        Ok(())
    }
}
