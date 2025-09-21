use crate::actions::action::GameActionHandler;
use crate::data::dialogue::id::DialogueID;
use crate::error::GameResult;
use crate::events::GameEvents;
use crate::state::GameState;

#[derive(Debug)]
pub struct TriggerDialogueAction {
    pub dialogue_id: DialogueID,
}

impl GameActionHandler for TriggerDialogueAction {
    fn handle(self, state: &mut GameState, _bus: &mut GameEvents) -> GameResult<()> {
        state.trigger_dialogue(self.dialogue_id);
        Ok(())
    }
}
