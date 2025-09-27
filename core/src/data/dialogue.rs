use crate::data::avatar::id::AvatarID;
use crate::data::dialogue::action::DialogueAction;
use crate::data::dialogue::entry::{DialogueEntry, DialogueEntryBuilder};
use crate::data::dialogue::options::DialogueOptions;
use serde::{Deserialize, Serialize};

pub mod action;
pub mod condition;
pub mod data;
pub mod entry;
pub mod event;
pub mod id;
pub mod options;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Dialogue {
    pub entries: Vec<DialogueEntry>,
}

impl Dialogue {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn builder() -> DialogueBuilder {
        DialogueBuilder::new()
    }
}

#[derive(Debug, Default)]
pub struct DialogueBuilder {
    entries: Vec<DialogueEntry>,
    avatar_id: AvatarID,
    avatar_name: Option<String>,
    options: DialogueOptions,
}

impl DialogueBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Dialogue {
        Dialogue {
            entries: self.entries,
        }
    }

    pub fn avatar(mut self, avatar_id: AvatarID) -> Self {
        self.avatar_id = avatar_id;
        self
    }

    pub fn avatar_name(mut self, name: &str) -> Self {
        self.avatar_name = Some(name.to_string());
        self
    }

    pub fn options(mut self, options: DialogueOptions) -> Self {
        self.options = options;
        self
    }

    pub fn entry(
        mut self,
        text: impl Into<String>,
        f: impl FnOnce(DialogueEntryBuilder) -> DialogueEntryBuilder,
    ) -> Self {
        let entry = f(DialogueEntryBuilder::new()
            .text(text)
            .avatar_id(self.avatar_id)
            .avatar_name(self.avatar_name.clone().unwrap_or("???".to_string()))
            .options(self.options))
        .build();
        self.entries.push(entry);
        self
    }

    pub fn step(self, text: impl Into<String>, button: impl Into<String>) -> Self {
        self.entry(text, |e| e.add_action(DialogueAction::new(button).jump(1)))
    }

    pub fn end(self, text: impl Into<String>, button: impl Into<String>) -> Self {
        self.entry(text, |e| e.end(button))
    }
}
