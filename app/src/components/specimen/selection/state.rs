use crate::app::GameApp;
use crate::components::column_config::SortedSpecimenTableColumnConfig;
use crate::components::options::SpecimenSelectionOptions;
use dungeon_breeder_core::state::specimen::collection::SpecimenCollectionSort;
use dungeon_breeder_core::state::specimen::SpecimenId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecimenSelectionState {
    pub options: SpecimenSelectionOptions,
    pub sort: SpecimenCollectionSort,
    pub columns: SortedSpecimenTableColumnConfig,
    #[serde(skip, default)]
    pub sorted_ids: Vec<SpecimenId>,
    #[serde(skip, default = "default_true")]
    sort_dirty: bool,
    #[serde(skip, default)]
    last_specimen_count: usize,
}

impl Default for SpecimenSelectionState {
    fn default() -> Self {
        Self {
            options: Default::default(),
            sort: Default::default(),
            columns: Default::default(),
            sorted_ids: vec![],
            sort_dirty: true,
            last_specimen_count: 0,
        }
    }
}

impl SpecimenSelectionState {
    pub fn update(&mut self, app: &mut GameApp) {
        // Force update when Specimen are added/removed
        let specimen_count = app.game.state.specimen.len();
        if specimen_count != self.last_specimen_count {
            self.sort_dirty();
            self.last_specimen_count = specimen_count;
        }

        if self.sort_dirty {
            self.sort_dirty = false;
            self.sort(app);
        }
    }

    pub fn sort_dirty(&mut self) {
        self.sort_dirty = true;
    }

    pub fn clear(&mut self) {
        self.sorted_ids.clear();
        self.sort_dirty = true;
    }

    pub fn selected_specimen_id(&self) -> Option<SpecimenId> {
        self.options.selected_specimen_id
    }

    fn sort(&mut self, app: &mut GameApp) {
        self.update_excluded_ids(app);
        self.sorted_ids = app.game.state.specimen.sorted_ids(&self.sort);
        if let Some(excluded_id) = self.options.excluded_specimen {
            self.sorted_ids.retain(|id| *id != excluded_id);
        }
    }

    fn update_excluded_ids(&mut self, app: &mut GameApp) {
        self.sort.excluded_ids.clear();

        if self.options.exclude_specimen_assigned_to_dungeon_layer_slot {
            self.sort
                .excluded_ids
                .extend(app.game.state.dungeon.iter_layer_slot_assigned_specimen());
        }

        if self.options.exclude_specimen_on_breeding_cooldown {
            self.sort.excluded_ids.extend(
                app.game
                    .state
                    .specimen
                    .iter_on_breeding_cooldown()
                    .map(|specimen| specimen.id),
            );
        }
    }
}

fn default_true() -> bool {
    true
}
