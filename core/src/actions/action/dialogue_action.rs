use crate::actions::action::GameActionHandler;
use crate::error::{GameError, GameResult};
use crate::events::GameEvents;
use crate::state::GameState;

#[derive(Debug)]
pub struct TakeDialogueAction {
    pub action_index: usize,
}

impl GameActionHandler for TakeDialogueAction {
    fn handle(self, state: &mut GameState, bus: &mut GameEvents) -> GameResult<()> {
        let active_dialogue = state
            .active_dialogue
            .as_mut()
            .ok_or(GameError::NoActiveDialogue)?;

        let events = active_dialogue.take_action(self.action_index)?;
        state.handle_dialogue_events(bus, events);

        Ok(())
    }
}
