use crate::app::GameApp;
use crate::components::{Component, EnumSelect, SortedSpecimenTable};
use crate::modals::AppModal;
use dungeon_breeder_core::state::specimen::collection::SpecimenCollectionSort;
use dungeon_breeder_core::state::specimen::SpecimenId;
use egui::{Context, Id, Ui};
use serde::{Deserialize, Serialize};

#[allow(clippy::type_complexity)]
#[derive(Default, Serialize, Deserialize)]
pub struct SpecimenSelectionModal {
    selected_specimen_id: Option<SpecimenId>,
    sort_config: SpecimenCollectionSort,
    #[serde(skip, default)]
    callback: Option<Box<dyn Fn(Option<SpecimenId>, &mut GameApp)>>,
    #[serde(skip, default)]
    sorted_ids: Vec<SpecimenId>,
    #[serde(skip, default = "default_true")]
    sort_dirty: bool,
}

impl SpecimenSelectionModal {
    pub fn open(
        &mut self,
        selected_specimen_id: Option<SpecimenId>,
        callback: impl Fn(Option<SpecimenId>, &mut GameApp) + 'static,
    ) {
        self.selected_specimen_id = selected_specimen_id;
        self.callback = Some(Box::new(callback));
    }

    fn select(&mut self, specimen_id: Option<SpecimenId>, app: &mut GameApp) {
        if let Some(callback) = self.callback.take() {
            callback(specimen_id, app);
        }
    }

    fn sort(&mut self, app: &mut GameApp) {
        self.sorted_ids = app.game.state.specimen.sorted_ids(&self.sort_config);
    }
}

impl AppModal for SpecimenSelectionModal {
    fn id(&self) -> Id {
        Id::new("specimen_selection_modal")
    }

    fn is_open(&self) -> bool {
        self.callback.is_some()
    }

    fn close(&mut self) {
        self.callback = None;
    }

    fn before_close(&mut self, _ctx: &Context, app: &mut GameApp) {
        self.sorted_ids.clear();
        self.sort_dirty = true;
    }

    fn update_content(&mut self, ui: &mut Ui, app: &mut GameApp) {
        if self.sort_dirty {
            self.sort_dirty = false;
            self.sort(app);
        }

        let old_sort_field = self.sort_config.sort_field;
        let old_sort_direction = self.sort_config.sort_direction;
        ui.horizontal(|ui| {
            EnumSelect::new(&mut self.sort_config.sort_field, "select_sort_field").ui(ui);
            EnumSelect::new(
                &mut self.sort_config.sort_direction,
                "select_sort_direction",
            )
            .ui(ui);
        });

        if self.sort_config.sort_field != old_sort_field
            || self.sort_config.sort_direction != old_sort_direction
        {
            self.sort_dirty = true;
        }

        ui.separator();

        SortedSpecimenTable::new(
            &app.game.state.specimen,
            &self.sorted_ids,
            &mut self.selected_specimen_id,
        )
        .ui(ui);

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Select").clicked() {
                self.select(self.selected_specimen_id, app);
            }
            if ui.button("Clear").clicked() {
                self.select(None, app);
            }
        });
    }
}

fn default_true() -> bool {
    true
}
