use crate::data::dialogue::action::DialogueAction;
use crate::data::dialogue::entry::DialogueEntry;
use crate::data::dialogue::event::DialogueEvent;
use crate::data::dialogue::id::DialogueID;
use crate::data::dialogue::Dialogue;
use crate::error::{GameError, GameResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DialogueState {
    pub dialogue: Option<Dialogue>,
    pub index: usize,
}

impl DialogueState {
    pub fn get_entry(&self) -> GameResult<&DialogueEntry> {
        let Some(dialogue) = &self.dialogue else {
            return Err(GameError::NoActiveDialogue);
        };

        dialogue
            .entries
            .get(self.index)
            .ok_or(GameError::dialogue_out_of_bounds(self.index))
    }

    pub fn from_dialogue_id(dialogue_id: DialogueID) -> Self {
        Self {
            dialogue: Some(dialogue_id.get_dialogue()),
            index: 0,
        }
    }

    pub fn get_action(&self, action_index: usize) -> GameResult<&DialogueAction> {
        let entry = self.get_entry()?;
        entry
            .actions
            .get(action_index)
            .ok_or(GameError::dialogue_action_out_of_bounds(
                self.index,
                action_index,
            ))
    }

    pub fn handle_dialogue_event(&mut self, event: &DialogueEvent) {
        match event {
            DialogueEvent::End => self.dialogue = None,
            DialogueEvent::Jump(relative) => {
                if *relative >= 0 {
                    self.index = self.index.saturating_add(*relative as usize);
                } else {
                    self.index = self.index.saturating_sub(relative.unsigned_abs() as usize);
                }
            }
            _ => {}
        }
    }

    pub fn force_bg_interactive(&mut self) {
        let Some(dialogue) = &mut self.dialogue else {
            return;
        };

        let Some(entry) = dialogue.entries.get_mut(self.index) else {
            return;
        };

        entry.options.allow_bg_interactions = true;
    }
}
