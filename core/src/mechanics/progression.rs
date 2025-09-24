use crate::data::dialogue::id::DialogueID;
use crate::state::GameState;
use crate::types::flag::GameFlag;

pub fn check_progression(state: &mut GameState) {
    check_tutorial_complete(state);
}

fn check_tutorial_complete(state: &mut GameState) {
    if state.active_dialogue.dialogue.is_none() && !state.flags.get(GameFlag::TutorialComplete) {
        state.trigger_dialogue(DialogueID::Tutorial)
    }
}
