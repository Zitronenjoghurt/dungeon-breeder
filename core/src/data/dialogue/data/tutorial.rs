use crate::data::avatar::id::AvatarID;
use crate::data::dialogue::Dialogue;
use crate::events::event::GameEvent;
use crate::feedback::GameFeedback;

pub fn build_tutorial() -> Dialogue {
    Dialogue::builder()
        .avatar(AvatarID::Advisor)
        .avatar_name("Some wise being")
        .entry("An unknown presence stirs before me... Tell me, wanderer, what name do you carry through these realms?", |e| {
            e.jump("The researcher", 6)
                .jump("Your worst nightmare", 1)
                .jump("...?", 5)
        })
        .entry(
            "Darkness cloaks your intent, wretched fiend. If malice guides your steps, then let the nine hells reclaim what they have spawned!",
            |e| {
                e.action("Try me", |a| {
                    a.game_event(GameEvent::DoFeedback(GameFeedback::CloseApp))
                        .end()
                })
                .action("Nooooo!", |a| {
                    a.game_event(GameEvent::DoFeedback(GameFeedback::CloseApp))
                        .end()
                })
                .jump("I LIED!", 1)
            },
        )
        .entry("Your essence remains veiled... Speak true, traveler. Who walks this path before me?", |e| {
            e.jump("The Researcher", 4)
                .jump("Your worst nightmare", 1)
                .jump("...?", 3)
        })
        .entry("Have years of study taught me nothing, that you would question my judgment so readily?", |e|
            e.jump("Yes (idk what that means)", 1)
        )
        .entry("Then perhaps you have come to the wrong place. The town has no need for those who seek conflict over understanding.", |e| {
            e.action("Ok", |a| {
                a.game_event(GameEvent::DoFeedback(GameFeedback::CloseApp))
                    .end()
            })
                .action("Wait!", |a| {
                    a.game_event(GameEvent::DoFeedback(GameFeedback::CloseApp))
                        .end()
                })
        })
        .entry(
            "Your manner is... peculiar. Let me ask plainly then, are you THE researcher we have been expecting?",
            |e| e.jump("Yes", 1),
        )
        .end("Many claim to be what they are not. Tell me, what purpose did lead you to us?", "(WIP)")
        .build()
}
