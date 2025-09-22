use crate::actions::action::GameActionHandler;
use crate::error::GameResult;
use crate::events::GameEvents;
use crate::state::GameState;

#[derive(Debug)]
pub struct TakeDialogueAction {
    pub action_index: usize,
}

impl GameActionHandler for TakeDialogueAction {
    fn handle(self, state: &mut GameState, bus: &mut GameEvents) -> GameResult<()> {
        let events = state
            .active_dialogue
            .get_action_events(self.action_index)?
            .to_vec();
        state.handle_dialogue_events(bus, &events);

        Ok(())
    }
}
