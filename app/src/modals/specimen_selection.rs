use crate::app::GameApp;
use crate::components::options::SpecimenSelectionOptions;
use crate::components::state::SpecimenSelectionState;
use crate::components::{Component, SpecimenSelection};
use crate::modals::AppModal;
use dungeon_breeder_core::state::specimen::SpecimenId;
use egui::{Context, Id, Ui};
use serde::{Deserialize, Serialize};

#[allow(clippy::type_complexity)]
#[derive(Default, Serialize, Deserialize)]
pub struct SpecimenSelectionModal {
    selection_state: SpecimenSelectionState,
    #[serde(skip, default)]
    callback: Option<Box<dyn Fn(Option<SpecimenId>, &mut GameApp)>>,
}

impl SpecimenSelectionModal {
    pub fn open(
        &mut self,
        options: SpecimenSelectionOptions,
        callback: impl Fn(Option<SpecimenId>, &mut GameApp) + 'static,
    ) {
        self.selection_state.options = options;
        self.selection_state.sort_dirty();
        self.callback = Some(Box::new(callback));
    }

    fn select(&mut self, specimen_id: Option<SpecimenId>, app: &mut GameApp) {
        if let Some(callback) = self.callback.take() {
            callback(specimen_id, app);
        }
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

    fn before_close(&mut self, _ctx: &Context, _app: &mut GameApp) {
        self.selection_state.clear();
    }

    fn update_content(&mut self, ui: &mut Ui, app: &mut GameApp) {
        ui.vertical_centered_justified(|ui| {
            ui.heading("Select specimen");
        });

        ui.separator();

        SpecimenSelection::new(app, &mut self.selection_state).ui(ui);

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Select").clicked() {
                self.select(self.selection_state.selected_specimen_id(), app);
            }
            if ui.button("Clear").clicked() {
                self.select(None, app);
            }
        });
    }
}
