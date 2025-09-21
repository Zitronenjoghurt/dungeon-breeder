use crate::data::character::id::CharacterID;
use crate::data::dialogue::event::DialogueEvent;

#[derive(Debug)]
pub struct DialogueEntry<'a> {
    pub character_id: CharacterID,
    pub text: &'a str,
    pub actions: &'a [DialogueEntryAction<'a>],
}

#[derive(Debug)]
pub struct DialogueEntryAction<'a> {
    pub text: &'a str,
    pub events: &'a [DialogueEvent],
}

#[macro_export]
macro_rules! dialogue_entry {
    (simple: $char_id:expr, $text:literal) => {
        $crate::data::dialogue::entry::DialogueEntry {
            character_id: $char_id,
            text: $text,
            actions: &[dialogue_action!(step: "Ok")],
        }
    };

    (step: $char_id:expr, $text:literal, $continue_text:literal) => {
        $crate::data::dialogue::entry::DialogueEntry {
            character_id: $char_id,
            text: $text,
            actions: &[dialogue_action!(step: $continue_text)],
        }
    };

    ($char_id:expr, $text:literal => [$($action:tt)*]) => {
        $crate::data::dialogue::entry::DialogueEntry {
            character_id: $char_id,
            text: $text,
            actions: &[$($action)*],
        }
    };
}

#[macro_export]
macro_rules! dialogue_action {
    (step: $text:literal) => {
        $crate::data::dialogue::entry::DialogueEntryAction {
            text: $text,
            events: &[$crate::dialogue_event!(step)],
        }
    };

    (jump: $text:literal => $offset:expr) => {
        $crate::data::dialogue::entry::DialogueEntryAction {
            text: $text,
            events: &[$crate::dialogue_event!(jump: $offset)],
        }
    };

    ($text:literal => [$($event:expr),* $(,)?]) => {
        $crate::data::dialogue::entry::DialogueEntryAction {
            text: $text,
            events: &[$($event),*],
        }
    };
}
