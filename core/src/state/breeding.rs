use crate::events::event::GameEvent;
use crate::state::specimen::collection::SpecimenCollection;
use crate::state::specimen::SpecimenId;
use crate::types::specimen_stat::SpecimenStat;
use crate::types::trend::Trend;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

pub mod stat_trends;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BreedingState {
    last_parent_id_1: Option<SpecimenId>,
    last_parent_id_2: Option<SpecimenId>,
    last_offspring_id: Option<SpecimenId>,
}

impl BreedingState {
    pub fn last_parent_id_1(&self) -> Option<SpecimenId> {
        self.last_parent_id_1
    }

    pub fn last_parent_id_2(&self) -> Option<SpecimenId> {
        self.last_parent_id_2
    }

    pub fn last_offspring_id(&self) -> Option<SpecimenId> {
        self.last_offspring_id
    }

    pub fn on_successful_breed(
        &mut self,
        parent_1_id: SpecimenId,
        parent_2_id: SpecimenId,
        new_specimen_id: SpecimenId,
    ) {
        self.last_parent_id_1 = Some(parent_1_id);
        self.last_parent_id_2 = Some(parent_2_id);
        self.last_offspring_id = Some(new_specimen_id);
    }

    pub fn get_stat_trends(
        &self,
        collection: &SpecimenCollection,
    ) -> Option<stat_trends::BreedingStatTrends> {
        let parent_1 = self
            .last_parent_id_1
            .and_then(|id| collection.get_by_id(id))?;
        let parent_2 = self
            .last_parent_id_2
            .and_then(|id| collection.get_by_id(id))?;
        let offspring = self
            .last_offspring_id
            .and_then(|id| collection.get_by_id(id))?;

        let mut trends = stat_trends::BreedingStatTrends::default();
        for stat in SpecimenStat::iter() {
            let parent_1_value = parent_1.get_stat(&stat);
            let parent_2_value = parent_2.get_stat(&stat);
            let parent_avg = (parent_1_value + parent_2_value) / 2.0;
            let offspring_value = offspring.get_stat(&stat);

            if offspring_value > parent_1_value && offspring_value > parent_2_value {
                trends.set_stat(&stat, Trend::FarUpwards);
            } else if offspring_value < parent_1_value && offspring_value < parent_2_value {
                trends.set_stat(&stat, Trend::FarDownwards);
            } else if offspring_value > parent_avg {
                trends.set_stat(&stat, Trend::Upwards);
            } else if offspring_value < parent_avg {
                trends.set_stat(&stat, Trend::Downwards);
            } else {
                trends.set_stat(&stat, Trend::Stable);
            }
        }

        Some(trends)
    }
}

// Events
impl BreedingState {
    #[tracing::instrument(
        target = "game",
        name = "game::state::breeding::handle_event",
        level = "trace",
        skip(self)
    )]
    pub fn handle_event(&mut self, event: &GameEvent) {
        if let GameEvent::SpecimenBred(event) = event {
            self.on_successful_breed(event.parent_1_id, event.parent_2_id, event.specimen_id);
        }
    }
}
