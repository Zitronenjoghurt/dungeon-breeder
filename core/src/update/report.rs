use crate::actions::report::GameActionReport;
use crate::data::item::id::ItemID;
use crate::events::event::GameEvent;
use crate::feedback::GameFeedback;
use std::collections::BTreeMap;
use std::time::Duration;

#[derive(Debug, Default)]
pub struct GameUpdateReport {
    pub action_report: GameActionReport,
    pub feedback: Vec<GameFeedback>,
    pub progress_report: GameUpdateProgressReport,
    pub ticks_elapsed: u64,
    pub time_elapsed: Duration,
}

impl GameUpdateReport {
    pub fn handle_event(&mut self, event: &GameEvent) {
        self.progress_report.handle_event(event);

        if let GameEvent::DoFeedback(feedback) = &event {
            self.feedback.push(feedback.clone());
        }
    }
}

#[derive(Debug, Default)]
pub struct GameUpdateProgressReport {
    pub items: BTreeMap<ItemID, u64>,
    pub times_specimen_slain: u64,
}

impl GameUpdateProgressReport {
    pub fn handle_event(&mut self, event: &GameEvent) {
        match event {
            GameEvent::ItemObtained(event) => self.handle_add_item(event.item_id, event.amount),
            GameEvent::SpecimenSlain(_) => self.times_specimen_slain += 1,
            _ => {}
        }
    }

    fn handle_add_item(&mut self, item_id: ItemID, amount: u64) {
        *self.items.entry(item_id).or_insert(0) += amount;
    }
}
