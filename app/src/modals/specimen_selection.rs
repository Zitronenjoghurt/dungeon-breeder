use crate::app::GameApp;
use crate::components::{Component, SpecimenDropdownSelection};
use crate::modals::AppModal;
use dungeon_breeder_core::state::specimen::SpecimenId;
use egui::{Context, Id, Ui};
use serde::{Deserialize, Serialize};

#[allow(clippy::type_complexity)]
#[derive(Default, Serialize, Deserialize)]
pub struct SpecimenSelectionModal {
    selected_specimen_id: SpecimenId,
    #[serde(skip, default)]
    callback: Option<Box<dyn Fn(Option<SpecimenId>, &mut GameApp)>>,
}

impl SpecimenSelectionModal {
    pub fn open(&mut self, callback: impl Fn(Option<SpecimenId>, &mut GameApp) + 'static) {
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

    fn before_close(&mut self, _ctx: &Context, app: &mut GameApp) {
        self.select(None, app);
    }

    fn update_content(&mut self, ui: &mut Ui, app: &mut GameApp) {
        SpecimenDropdownSelection::new(&app.game.state.specimen, &mut self.selected_specimen_id)
            .ui(ui);
        if ui.button("Select").clicked() {
            self.select(Some(self.selected_specimen_id), app);
        }
    }
}
