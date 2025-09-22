use crate::data::dialogue::event::DialogueEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DialogueAction {
    pub text: String,
    pub events: Vec<DialogueEvent>,
}

impl DialogueAction {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            events: Vec::new(),
        }
    }

    pub fn event(mut self, event: DialogueEvent) -> Self {
        self.events.push(event);
        self
    }

    pub fn step(text: &str) -> Self {
        DialogueAction::new(text).event(DialogueEvent::step())
    }
}
