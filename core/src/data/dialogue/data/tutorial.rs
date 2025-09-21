use crate::data::character::id::CharacterID;
use crate::data::dialogue::Dialogue;
use crate::{dialogue, dialogue_action, dialogue_entry, dialogue_event};

pub static DIALOGUE_TUTORIAL: Dialogue = dialogue!(
    dialogue_entry!(simple: CharacterID::Advisor, "Hello, world!"),
    dialogue_entry!(simple: CharacterID::Advisor, "This is a test to see if stuff works."),
    dialogue_entry!(
        CharacterID::Advisor,
        "Do you want to start?" => [
            dialogue_action!("Yes" => [dialogue_event!(jump: 1)]),
            dialogue_action!("No" => [dialogue_event!(jump: 0)]),
        ]
    )
);
