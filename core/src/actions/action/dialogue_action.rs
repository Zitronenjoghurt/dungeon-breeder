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
        let action = state.active_dialogue.get_action(self.action_index)?;
        if !action.conditions_met(state) {
            return Err(GameError::DialogueActionConditionNotMet {
                index: state.active_dialogue.index,
                action_index: self.action_index,
            });
        }

        let events = action.events.clone();
        state.handle_dialogue_events(bus, &events);

        Ok(())
    }
}
