use crate::data::avatar::id::AvatarID;
use crate::data::creature::id::CreatureID;
use crate::data::dialogue::condition::DialogueCondition;
use crate::data::dialogue::event::DialogueEvent;
use crate::data::dialogue::options::{DialogueOptions, DialoguePosition};
use crate::data::dialogue::DialogueBuilder;
use crate::events::event::GameEvent;
use crate::feedback::GameFeedback;
use crate::state::specimen::obtain_method::SpecimenObtainMethod;
use crate::state::specimen::NewSpecimen;
use crate::types::flag::GameFlag;

pub fn build_tutorial(builder: DialogueBuilder) -> DialogueBuilder {
    builder
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
            e.jump("Yes (idk what that means)", 1),
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
        .entry("Many claim to be what they are not. Tell me, what purpose did lead you to us?", |e|
            e.action("I don't know", |a| a.jump(1))
                .action("The mayor's request", |a| a.jump(3))
                .action("The recent anomaly", |a| a.jump(3)),
        )
        .entry("To wander without knowing one's path... Tell me, did no herald's word reach you? No sealed correspondence bearing our seal?", |e|
            e.jump("Yes (show him your letter)", 2)
                .jump("No", 1),
        )
        .entry("Then this grows stranger still... An uninvited researcher, knowing nothing of our plight, yet arriving precisely when our need grows most dire.", |e|
            e.jump("Yep", 1)
                .jump("That's just how I am", 1),
        )
        .entry("Then you must know, or soon shall learn, that two peculiar manifestations have emerged within our very town. Orb-like aberrations, pulsing with unnatural life... We dare not approach, for their nature remains shrouded in mystery.", |e|
            e.jump("Peculiar...", 1)
                .jump("I don't understand", 1),
        )
        .entry("Ah, but where are my manners in these troubled times? I am Aegus, advisor to our mayor, keeper of what little wisdom these old bones still carry.", |e|
            e.jump("Nice to meet you", 1),
        )
        .avatar_name("Aegus")
        .entry("Your task, should you accept this burden, is to unravel the nature of these orb-like entities. Yet research demands resources, and our town's treasury could not fund a proper feast, let alone scholarly pursuits.", |e|
            e.jump("But I am broke as heck", 1),
        )
        .entry("Yet perhaps this mutual poverty itself reveals the path forward. The Adventurers' Guild hungers for purpose, for trials to test their mettle. There exists an ancient practice, one might call it... controversial, whereby one constructs labyrinths of challenge, chambers of ordeal where adventurers pay tribute to face their fears... A dungeon.", |e|
            e.jump("Alright!", 1)
                .jump("I am a researcher!", 1),
        )
        .entry("I would not suggest such a perilous undertaking, were it not for a most troubling discovery... these orbs seem inexorably drawn to places of conflict and trial. As if they feed upon the very essence of challenge itself.", |e|
            e.action_text("Got it!")
                .action_text("Fine...")
                .event(DialogueEvent::SetFlag(GameFlag::UnlockedTopBar))
                .event(DialogueEvent::GameEvent(GameEvent::DoSpawnSpecimen(NewSpecimen {
                    creature_id: CreatureID::Gonk,
                    obtain_method: SpecimenObtainMethod::Unknown,
                    strength: 0.73,
                    intelligence: 0.72,
                    vitality: 0.75,
                    agility: 0.78,
                    regeneration: 0.84,
                    fertility: 0.82,
                    breeding_generation: 1,
                    fusion_generation: 1,
                })))
                .event(DialogueEvent::GameEvent(GameEvent::DoSpawnSpecimen(NewSpecimen {
                    creature_id: CreatureID::Gonk,
                    obtain_method: SpecimenObtainMethod::Unknown,
                    strength: 0.71,
                    intelligence: 0.75,
                    vitality: 0.73,
                    agility: 0.79,
                    regeneration: 0.72,
                    fertility: 0.91,
                    breeding_generation: 1,
                    fusion_generation: 1,
                })))
                .event(DialogueEvent::Jump(1)),
        )
        .options(DialogueOptions {
            allow_bg_interactions: true,
            position: DialoguePosition::TopRight,
        })
        .entry("Allow me a moment to explain the fundamentals. Above, you will find the various aspects of your work, ancient mechanisms abandoned by prior researchers. Their full purpose still eludes me, though I have gleaned enough to guide you.", |e|
            e.action_text("Understood")
                .action_text("I am looking")
                .event(DialogueEvent::SetFlag(GameFlag::UnlockedSpecimenOverview))
                .event(DialogueEvent::Jump(1)),
        )
        .entry("The leftmost tool seems to track those peculiar orbs somehow. It displays information about them, though the symbols and numbers it shows are utterly foreign to me.", |e|
            e.action("Ok (open the specimen overview)", |a|
                a.condition(DialogueCondition::FlagSet(GameFlag::HasClickedSpecimenOverview))
                    .jump(1)
            )
        )
}
