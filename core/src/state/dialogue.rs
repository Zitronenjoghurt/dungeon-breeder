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

    pub fn get_action_events(&self, action_index: usize) -> GameResult<&[DialogueEvent]> {
        let entry = self.get_entry()?;
        let action =
            entry
                .actions
                .get(action_index)
                .ok_or(GameError::dialogue_action_out_of_bounds(
                    self.index,
                    action_index,
                ))?;

        Ok(&action.events)
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
}
