use crate::data::dialogue::condition::DialogueCondition;
use crate::data::dialogue::event::DialogueEvent;
use crate::events::event::GameEvent;
use crate::state::GameState;
use crate::types::flag::GameFlag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DialogueAction {
    pub text: String,
    pub events: Vec<DialogueEvent>,
    pub conditions: Vec<DialogueCondition>,
}

impl DialogueAction {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            events: Vec::new(),
            conditions: Vec::new(),
        }
    }

    pub fn conditions_met(&self, state: &GameState) -> bool {
        self.conditions.iter().all(|c| c.check(state))
    }

    pub fn add_event(&mut self, event: DialogueEvent) {
        self.events.push(event);
    }

    pub fn event(mut self, event: DialogueEvent) -> Self {
        self.events.push(event);
        self
    }

    pub fn condition(mut self, condition: DialogueCondition) -> Self {
        self.conditions.push(condition);
        self
    }

    pub fn jump(self, amount: i16) -> Self {
        self.event(DialogueEvent::Jump(amount))
    }

    pub fn end(self) -> Self {
        self.event(DialogueEvent::End)
    }

    pub fn game_event(self, event: GameEvent) -> Self {
        self.event(DialogueEvent::GameEvent(event))
    }

    pub fn set_flag(self, flag: GameFlag) -> Self {
        self.event(DialogueEvent::SetFlag(flag))
    }
}
