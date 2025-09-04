use crate::error::GameResult;
use crate::state::item::collection::ItemCollection;
use crate::state::specimen::collection::SpecimenCollection;
use crate::state::specimen::SpecimenId;
use crate::state::timer::Timer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DungeonLayerSlot {
    assigned_specimen: Option<SpecimenId>,
    slay_duration_secs: u64,
    regeneration_duration_secs: u64,
    timer: Timer,
    is_regenerating: bool,
}

impl DungeonLayerSlot {
    pub fn tick(
        &mut self,
        specimen_collection: &mut SpecimenCollection,
        items: &mut ItemCollection,
    ) {
        let Some(specimen_id) = self.assigned_specimen else {
            return;
        };

        let Some(specimen) = specimen_collection.get_by_id_mut(specimen_id) else {
            self.assigned_specimen = None;
            return;
        };

        self.slay_duration_secs = specimen.slay_duration_secs();
        self.regeneration_duration_secs = specimen.slay_duration_secs();

        let max_secs_current = if self.is_regenerating {
            self.regeneration_duration_secs
        } else {
            self.slay_duration_secs
        };

        if !self.timer.tick(max_secs_current) {
            return;
        }

        if self.is_regenerating {
            self.is_regenerating = false;
        } else {
            self.is_regenerating = true;
            let dropped_items = specimen.generate_drops();
            items.add_new_batch(&dropped_items);
            specimen.on_slain();
        }
    }

    pub fn get_assigned_specimen_id(&self) -> Option<SpecimenId> {
        self.assigned_specimen
    }

    pub fn set_assigned_specimen_id(&mut self, specimen_id: Option<SpecimenId>) -> GameResult<()> {
        self.timer.reset();
        self.assigned_specimen = specimen_id;
        Ok(())
    }

    pub fn progress(&self) -> f32 {
        self.timer.progress(self.slay_duration_secs)
    }

    pub fn format_time_left(&self) -> String {
        self.timer.format_time_left(self.slay_duration_secs)
    }

    pub fn slay_duration_secs(&self) -> u64 {
        self.slay_duration_secs
    }

    pub fn regeneration_duration_secs(&self) -> u64 {
        self.regeneration_duration_secs
    }

    pub fn is_regenerating(&self) -> bool {
        self.is_regenerating && self.assigned_specimen.is_some()
    }
}
