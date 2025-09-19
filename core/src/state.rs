use crate::state::breeding::BreedingState;
use crate::state::clock::Clock;
use crate::state::dungeon::Dungeon;
use crate::state::fusion::FusionState;
use crate::state::item::collection::ItemCollection;
use crate::state::statistics::GameStatistics;
use crate::state::treasury::Treasury;
use crate::state::update_report::GameStateUpdateReport;
use serde::{Deserialize, Serialize};
use specimen::collection::SpecimenCollection;

pub mod breeding;
mod clock;
pub mod dungeon;
pub mod fusion;
pub mod item;
pub mod specimen;
pub mod statistics;
mod timer;
mod treasury;
pub mod update_report;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameState {
    pub breeding: BreedingState,
    pub clock: Clock,
    pub dungeon: Dungeon,
    pub fusion: FusionState,
    pub items: ItemCollection,
    pub specimen: SpecimenCollection,
    pub statistics: GameStatistics,
    pub treasury: Treasury,
}

impl GameState {
    pub fn update(&mut self) -> GameStateUpdateReport {
        let mut update_report = GameStateUpdateReport::default();

        let seconds_passed = self.clock.update();
        for _ in 0..seconds_passed {
            self.tick(&mut update_report);
        }

        self.statistics.on_game_state_update(&update_report);

        update_report
    }

    pub fn tick(&mut self, report: &mut GameStateUpdateReport) {
        report.on_tick();

        self.dungeon
            .tick(report, &mut self.specimen, &mut self.items);
    }
}
