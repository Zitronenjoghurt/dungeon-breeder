use crate::components::{Component, SpecimenSelection};
use crate::modals::AppModal;
use crate::state::AppState;
use dungeon_breeder_core::state::specimen::SpecimenId;
use egui::{Id, Ui};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct SpecimenSelectionModal {
    selected_specimen_id: SpecimenId,
    #[serde(skip, default)]
    callback: Option<Box<dyn Fn(SpecimenId, &mut AppState)>>,
}

impl SpecimenSelectionModal {
    pub fn open(&mut self, callback: impl Fn(SpecimenId, &mut AppState) + 'static) {
        self.callback = Some(Box::new(callback));
    }

    fn select(&mut self, specimen_id: SpecimenId, state: &mut AppState) {
        if let Some(callback) = self.callback.take() {
            callback(specimen_id, state);
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

    fn update_content(&mut self, ui: &mut Ui, state: &mut AppState) {
        SpecimenSelection::new(&state.game.state.specimen, &mut self.selected_specimen_id).ui(ui);
        if ui.button("Select").clicked() {
            self.select(self.selected_specimen_id, state);
        }
    }
}
