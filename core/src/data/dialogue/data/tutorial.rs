use crate::data::avatar::id::AvatarID;
use crate::data::dialogue::action::DialogueAction;
use crate::data::dialogue::event::DialogueEvent;
use crate::data::dialogue::Dialogue;
use crate::data::flags::GameFlag;

pub fn build_tutorial() -> Dialogue {
    Dialogue::builder()
        .avatar(AvatarID::Advisor)
        .avatar_name("Some wise being?")
        .step("Hello, world!")
        .step("This is a test to see if stuff works.")
        .entry("Do you want to start now?", |e| {
            e.action(
                DialogueAction::new("Yes")
                    .event(DialogueEvent::SetFlag(GameFlag::TutorialComplete))
                    .event(DialogueEvent::End),
            )
            .action(DialogueAction::step("No"))
        })
        .entry("You're stupid.", |e| {
            e.action(DialogueAction::new("No"))
                .action(DialogueAction::new("No doubt").event(DialogueEvent::Jump(-1)))
        })
        .build()
}
