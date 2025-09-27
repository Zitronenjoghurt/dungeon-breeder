use crate::actions::action::GameActionHandler;
use crate::error::GameResult;
use crate::events::GameEvents;
use crate::state::GameState;

pub struct DebugDialogueBgInteractiveAction;

impl GameActionHandler for DebugDialogueBgInteractiveAction {
    fn handle(self, state: &mut GameState, _bus: &mut GameEvents) -> GameResult<()> {
        state.active_dialogue.force_bg_interactive();
        Ok(())
    }
}
