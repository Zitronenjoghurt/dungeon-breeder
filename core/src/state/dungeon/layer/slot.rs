use crate::error::GameResult;
use crate::state::item::collection::ItemCollection;
use crate::state::specimen::collection::SpecimenCollection;
use crate::state::specimen::{Specimen, SpecimenId};
use crate::state::timer::Timer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DungeonLayerSlot {
    assigned_specimen: Option<SpecimenId>,
    #[serde(skip, default)]
    timer: Timer,
    #[serde(skip, default)]
    is_regenerating: bool,
    #[serde(skip, default)]
    slay_duration_secs: u64,
    #[serde(skip, default)]
    regen_duration_secs: u64,
}

impl DungeonLayerSlot {
    fn update_state(&mut self, specimen: &Specimen) {
        self.is_regenerating = specimen.is_regenerating;
        self.timer = specimen.slay_regen_timer;
        self.slay_duration_secs = specimen.slay_duration_secs();
        self.regen_duration_secs = specimen.regeneration_duration_secs();
    }

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

        let max_secs_current = if specimen.is_regenerating {
            specimen.regeneration_duration_secs()
        } else {
            specimen.slay_duration_secs()
        };

        if !specimen.slay_regen_timer.tick(max_secs_current) {
            self.update_state(specimen);
            return;
        }

        if specimen.is_regenerating {
            specimen.is_regenerating = false;
        } else {
            specimen.is_regenerating = true;
            let dropped_items = specimen.generate_drops();
            items.add_new_batch(&dropped_items);
            specimen.on_slain();
        }

        self.update_state(specimen);
    }

    pub fn get_assigned_specimen_id(&self) -> Option<SpecimenId> {
        self.assigned_specimen
    }

    pub fn set_assigned_specimen_id(&mut self, specimen_id: Option<SpecimenId>) -> GameResult<()> {
        self.assigned_specimen = specimen_id;
        Ok(())
    }

    pub fn max_seconds_current(&self) -> u64 {
        if self.is_regenerating {
            self.regen_duration_secs
        } else {
            self.slay_duration_secs
        }
    }

    pub fn progress(&self) -> f32 {
        self.timer.progress(self.max_seconds_current())
    }

    pub fn format_time_left(&self) -> String {
        self.timer.format_time_left(self.max_seconds_current())
    }

    pub fn is_regenerating(&self) -> bool {
        self.is_regenerating && self.assigned_specimen.is_some()
    }
}
