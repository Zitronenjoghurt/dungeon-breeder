use crate::data::dialogue::event::DialogueEvent;
use crate::events::event::GameEvent;
use crate::types::flag::GameFlag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DialogueAction {
    pub text: String,
    pub events: Vec<DialogueEvent>,
}

impl DialogueAction {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            events: Vec::new(),
        }
    }

    pub fn event(mut self, event: DialogueEvent) -> Self {
        self.events.push(event);
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
