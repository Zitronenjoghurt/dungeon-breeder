use crate::data::character::id::CharacterID;
use crate::data::dialogue::Dialogue;
use crate::data::flags::GameFlag;
use crate::{dialogue, dialogue_action, dialogue_entry, dialogue_event};

pub static DIALOGUE_TUTORIAL: Dialogue = dialogue!(
    dialogue_entry!(simple: CharacterID::Advisor, "Hello, world!"),
    dialogue_entry!(simple: CharacterID::Advisor, "This is a test to see if stuff works."),
    dialogue_entry!(
        CharacterID::Advisor,
        "Do you want to start now?" => [
            dialogue_action!("Yes" => [dialogue_event!(set: GameFlag::TutorialComplete), dialogue_event!(end)]),
            dialogue_action!("No" => [dialogue_event!(jump: 0)]),
        ]
    ),
    dialogue_entry!(CharacterID::Advisor, "You're stupid." => [
        dialogue_action!("No" => [dialogue_event!(jump: 0)]),
        dialogue_action!("No doubt" => [dialogue_event!(jump: -1)]),
    ])
);
