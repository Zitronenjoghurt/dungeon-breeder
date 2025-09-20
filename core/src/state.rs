use crate::events::event::GameEvent;
use crate::events::GameEvents;
use crate::state::breeding::BreedingState;
use crate::state::dungeon::Dungeon;
use crate::state::fusion::FusionState;
use crate::state::item::collection::ItemCollection;
use crate::state::statistics::GameStatistics;
use crate::state::treasury::Treasury;
use serde::{Deserialize, Serialize};
use specimen::collection::SpecimenCollection;

pub mod breeding;
pub mod dungeon;
pub mod fusion;
pub mod item;
pub mod specimen;
pub mod statistics;
mod timer;
mod treasury;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameState {
    pub breeding: BreedingState,
    pub dungeon: Dungeon,
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

    pub fn on_ticks_elapsed(&mut self, ticks: u64) {
        self.statistics.on_ticks_elapsed(ticks);
    }
}
