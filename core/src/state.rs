use crate::data::dialogue::event::DialogueEvent;
use crate::data::dialogue::id::DialogueID;
use crate::events::event::GameEvent;
use crate::events::GameEvents;
use crate::state::breeding::BreedingState;
use crate::state::dialogue::DialogueState;
use crate::state::dungeon::Dungeon;
use crate::state::flags::GameFlags;
use crate::state::fusion::FusionState;
use crate::state::item::collection::ItemCollection;
use crate::state::statistics::GameStatistics;
use crate::state::treasury::Treasury;
use rand_distr::num_traits::Saturating;
use serde::{Deserialize, Serialize};
use specimen::collection::SpecimenCollection;

pub mod breeding;
pub mod dialogue;
pub mod dungeon;
pub mod flags;
pub mod fusion;
pub mod item;
pub mod specimen;
pub mod statistics;
mod timer;
mod treasury;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameState {
    pub active_dialogue: DialogueState,
    pub breeding: BreedingState,
    pub dungeon: Dungeon,
    pub flags: GameFlags,
    pub fusion: FusionState,
    pub items: ItemCollection,
    pub specimen: SpecimenCollection,
    pub statistics: GameStatistics,
    pub treasury: Treasury,
}

impl GameState {
    #[tracing::instrument(
        target = "game",
        name = "game::state::tick",
        level = "trace",
        skip(self, bus)
    )]
    pub fn tick(&mut self, bus: &mut GameEvents) {
        self.dungeon.tick(bus, &self.specimen);
    }

    #[tracing::instrument(
        target = "game",
        name = "game::state::handle_event",
        level = "trace",
        skip(self, bus)
    )]
    pub fn handle_event(&mut self, bus: &mut GameEvents, event: &GameEvent) {
        self.breeding.handle_event(event);
        self.fusion.handle_event(event);
        self.items.handle_event(bus, event);
        self.specimen.handle_event(bus, event);
        self.statistics.handle_event(event);
    }

    pub fn handle_dialogue_events(&mut self, bus: &mut GameEvents, events: &[DialogueEvent]) {
        let mut to_skip = 0;
        for event in events {
            if to_skip > 0 {
                to_skip -= 1;
                continue;
            }

            self.handle_dialogue_event(bus, &event);
            to_skip = event.count_events_skipped(&self.flags);

            if event.should_ignore_following_events() {
                break;
            }
        }
    }

    pub fn handle_dialogue_event(&mut self, _bus: &mut GameEvents, event: &DialogueEvent) {
        self.active_dialogue.handle_dialogue_event(event);
        match event {
            DialogueEvent::SetFlag(flag) => self.flags.set(*flag),
            DialogueEvent::TriggerDialogue(dialogue_id) => self.trigger_dialogue(*dialogue_id),
            DialogueEvent::Unset(flag) => self.flags.unset(*flag),
            _ => {}
        }
    }

    pub fn trigger_dialogue(&mut self, dialogue_id: DialogueID) {
        self.active_dialogue = DialogueState::from_dialogue_id(dialogue_id);
    }

    #[tracing::instrument(
        target = "game",
        name = "game::state::on_ticks_elapsed",
        level = "trace",
        skip(self)
    )]
    pub fn on_ticks_elapsed(&mut self, ticks: u64) {
        self.statistics.on_ticks_elapsed(ticks);
    }
}
