use crate::data::avatar::id::AvatarID;
use crate::data::dialogue::action::DialogueAction;
use crate::data::dialogue::entry::DialogueEntry;
use crate::data::dialogue::options::DialogueOptions;
use serde::{Deserialize, Serialize};

pub mod action;
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

    pub fn entry(mut self, entry: DialogueEntry) -> Self {
        self.entries.push(entry);
        self
    }

    pub fn step(self, avatar: AvatarID, text: &str) -> Self {
        self.entry(DialogueEntry::step(avatar, text))
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

    pub fn entry(mut self, text: &str, f: impl FnOnce(DialogueEntry) -> DialogueEntry) -> Self {
        let entry = f(DialogueEntry::new(self.avatar_id, text)
            .avatar_name(self.avatar_name.clone())
            .options(self.options));
        self.entries.push(entry);
        self
    }

    pub fn step(self, text: &str) -> Self {
        self.entry(text, |e| e.action(DialogueAction::step("Ok")))
    }
}
