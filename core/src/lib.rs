use crate::actions::GameActions;
use crate::clock::Clock;
use crate::data::config::CONFIG;
use crate::events::event::GameEvent;
use crate::events::GameEvents;
use crate::mechanics::progression::check_progression;
use crate::state::GameState;
use serde::{Deserialize, Serialize};
use update::report::GameUpdateReport;

pub mod actions;
mod clock;
pub mod data;
mod error;
mod events;
pub mod feedback;
mod mechanics;
pub mod state;
pub mod types;
pub mod update;
pub mod utils;

pub const VERSION_INDEX: u32 = 0;
pub const VERSION_NAME: &str = "0.1.0-alpha";

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub version: u32,
    pub state: GameState,
    clock: Clock,
    #[serde(skip, default)]
    pub actions: GameActions,
    #[serde(skip, default)]
    events: GameEvents,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            version: VERSION_INDEX,
            state: GameState::default(),
            clock: Clock::default(),
            actions: GameActions::default(),
            events: GameEvents::default(),
        }
    }
}

impl Game {
    #[tracing::instrument(target = "game", name = "game::update", level = "trace", skip(self))]
    pub fn update(&mut self) -> GameUpdateReport {
        let now = std::time::Instant::now();

        let mut report = GameUpdateReport {
            action_report: self.actions.handle(&mut self.state, &mut self.events),
            ..Default::default()
        };

        // Handling events twice here ensures that the events are handled fast
        // AND always after a tick (should multiple ticks be processed in a single update)
        self.handle_events(&mut report);

        let ticks_elapsed = self.clock.update();
        for _ in 0..ticks_elapsed {
            self.state.tick(&mut self.events);
            self.handle_events(&mut report);
        }

        check_progression(&mut self.state);

        self.state.on_ticks_elapsed(ticks_elapsed);
        report.ticks_elapsed = ticks_elapsed;

        let time_elapsed = now.elapsed();
        report.time_elapsed = time_elapsed;

        tracing::trace!(target: "game", "Game update: {ticks_elapsed} ticks");

        report
    }

    #[tracing::instrument(
        target = "game",
        name = "game::handle_events",
        level = "trace",
        skip(self)
    )]
    fn handle_events(&mut self, report: &mut GameUpdateReport) {
        let mut generation = 0;

        while self.events.has_events() && generation < CONFIG.max_event_generations {
            generation += 1;
            for event in self.events.take_events() {
                self.handle_event(report, event);
            }
        }

        if generation == CONFIG.max_event_generations - 1 {
            eprintln!("Warning: reached max event generation");
        }
    }

    #[tracing::instrument(
        target = "game",
        name = "game::handle_event",
        level = "trace",
        skip(self)
    )]
    fn handle_event(&mut self, report: &mut GameUpdateReport, event: GameEvent) {
        if let GameEvent::DoSkipTime(event) = &event {
            self.clock.roll_back(event.seconds);
        }

        report.handle_event(&event);
        self.state.handle_event(&mut self.events, &event);
    }
}
