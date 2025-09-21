use crate::data::character::id::CharacterID;
use crate::data::dialogue::event::DialogueEvent;
use crate::data::dialogue::id::DialogueID;
use crate::error::{GameError, GameResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DialogueState {
    pub dialogue_id: DialogueID,
    pub index: usize,
}

impl DialogueState {
    pub fn from_dialogue_id(dialogue_id: DialogueID) -> Self {
        Self {
            dialogue_id,
            index: 0,
        }
    }

    pub fn get_action_texts(&self) -> GameResult<Vec<&'static str>> {
        let entry = self
            .dialogue_id
            .get_dialogue()
            .entries
            .get(self.index)
            .ok_or(GameError::dialogue_out_of_bounds(
                self.dialogue_id,
                self.index,
            ))?;

        Ok(entry.actions.iter().map(|action| action.text).collect())
    }

    pub fn get_character_id(&self) -> GameResult<CharacterID> {
        let entry = self
            .dialogue_id
            .get_dialogue()
            .entries
            .get(self.index)
            .ok_or(GameError::dialogue_out_of_bounds(
                self.dialogue_id,
                self.index,
            ))?;

        Ok(entry.character_id)
    }

    pub fn get_text(&self) -> GameResult<&'static str> {
        let entry = self
            .dialogue_id
            .get_dialogue()
            .entries
            .get(self.index)
            .ok_or(GameError::dialogue_out_of_bounds(
                self.dialogue_id,
                self.index,
            ))?;

        Ok(entry.text)
    }

    pub fn take_action(&self, action_index: usize) -> GameResult<&'static [DialogueEvent]> {
        let entry = self
            .dialogue_id
            .get_dialogue()
            .entries
            .get(self.index)
            .ok_or(GameError::dialogue_out_of_bounds(
                self.dialogue_id,
                self.index,
            ))?;

        let action =
            entry
                .actions
                .get(action_index)
                .ok_or(GameError::dialogue_action_out_of_bounds(
                    self.dialogue_id,
                    self.index,
                    action_index,
                ))?;

        Ok(action.events)
    }

    pub fn trigger_dialogue(&mut self, dialogue_id: DialogueID) {
        self.dialogue_id = dialogue_id;
        self.index = 0;
    }
}
