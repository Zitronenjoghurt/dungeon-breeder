use crate::data::avatar::id::AvatarID;
use crate::data::dialogue::action::DialogueAction;
use crate::data::dialogue::options::DialogueOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DialogueEntry {
    pub avatar_id: AvatarID,
    pub avatar_name: Option<String>,
    pub text: String,
    pub actions: Vec<DialogueAction>,
    pub options: DialogueOptions,
}

impl DialogueEntry {
    pub fn new(avatar_id: AvatarID, text: &str) -> Self {
        Self {
            avatar_id,
            avatar_name: None,
            text: text.to_string(),
            actions: Vec::new(),
            options: DialogueOptions::default(),
        }
    }

    pub fn avatar_name(mut self, name: Option<String>) -> Self {
        self.avatar_name = name;
        self
    }

    pub fn step(avatar_id: AvatarID, text: &str) -> Self {
        Self::new(avatar_id, text).action(DialogueAction::step("Ok"))
    }

    pub fn options(mut self, options: DialogueOptions) -> Self {
        self.options = options;
        self
    }

    pub fn action(mut self, action: DialogueAction) -> Self {
        self.actions.push(action);
        self
    }
}
