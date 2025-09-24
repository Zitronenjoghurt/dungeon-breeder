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

    pub fn builder() -> DialogueEntryBuilder {
        DialogueEntryBuilder::new()
    }
}

#[derive(Debug, Default)]
pub struct DialogueEntryBuilder {
    pub entry: DialogueEntry,
}

impl DialogueEntryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> DialogueEntry {
        self.entry
    }

    pub fn avatar_id(mut self, avatar_id: AvatarID) -> Self {
        self.entry.avatar_id = avatar_id;
        self
    }

    pub fn avatar_name(mut self, name: impl Into<String>) -> Self {
        self.entry.avatar_name = Some(name.into());
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.entry.text = text.into();
        self
    }

    pub fn action(
        mut self,
        text: impl Into<String>,
        f: impl FnOnce(DialogueAction) -> DialogueAction,
    ) -> Self {
        let action = f(DialogueAction::new(text));
        self.entry.actions.push(action);
        self
    }

    pub fn add_action(mut self, action: DialogueAction) -> Self {
        self.entry.actions.push(action);
        self
    }

    pub fn options(mut self, options: DialogueOptions) -> Self {
        self.entry.options = options;
        self
    }

    pub fn jump(self, text: impl Into<String>, amount: i16) -> Self {
        self.action(text, |a| a.jump(amount))
    }

    pub fn end(self, text: impl Into<String>) -> Self {
        self.action(text, |a| a.end())
    }
}
