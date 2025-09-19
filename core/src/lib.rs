use crate::actions::GameActions;
use crate::clock::Clock;
use crate::data::config::CONFIG;
use crate::events::event::GameEvent;
use crate::events::GameEvents;
use crate::state::GameState;
use crate::update_report::GameUpdateReport;
use serde::{Deserialize, Serialize};

pub mod actions;
mod clock;
pub mod data;
mod error;
mod events;
mod mechanics;
pub mod state;
pub mod types;
mod update_report;
mod utils;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Game {
    pub state: GameState,
    clock: Clock,
    #[serde(skip, default)]
    pub actions: GameActions,
    #[serde(skip, default)]
    events: GameEvents,
}

impl Game {
    pub fn update(&mut self) -> GameUpdateReport {
        let now = std::time::Instant::now();

        let action_report = self.actions.handle(&mut self.state, &mut self.events);

        // Handling events twice here ensures that the events are handled fast
        // AND always after a tick (should multiple ticks be processed in a single update)
        self.handle_events();

        let ticks_elapsed = self.clock.update();
        for _ in 0..ticks_elapsed {
            self.state.tick(&mut self.events);
            self.handle_events();
        }

        self.state.on_ticks_elapsed(ticks_elapsed);

        let time_elapsed = now.elapsed();

        GameUpdateReport {
            action_report,
            ticks_elapsed,
            time_elapsed,
        }
    }

    fn handle_events(&mut self) {
        let mut generation = 0;

        while self.events.has_events() && generation < CONFIG.max_event_generations {
            generation += 1;
            for event in self.events.take_events() {
                self.handle_event(event);
            }
        }

        if generation == CONFIG.max_event_generations - 1 {
            eprintln!("Warning: reached max event generation");
        }
    }

    fn handle_event(&mut self, event: GameEvent) {
        self.state.handle_event(&mut self.events, &event);
    }
}
