use crate::app::GameApp;
use crate::components::specimen::info::SpecimenInfo;
use crate::components::state::SpecimenSelectionState;
use crate::components::{Component, SpecimenSelection};
use egui::Ui;

pub struct SpecimenOverview<'a> {
    app: &'a mut GameApp,
    selection_state: &'a mut SpecimenSelectionState,
}

impl<'a> SpecimenOverview<'a> {
    pub fn new(app: &'a mut GameApp, selection_state: &'a mut SpecimenSelectionState) -> Self {
        Self {
            app,
            selection_state,
        }
    }
}

impl Component for SpecimenOverview<'_> {
    fn ui(self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if let Some(specimen_id) = self.selection_state.selected_specimen_id() {
                SpecimenInfo::new(
                    &mut self.app.textures,
                    self.app.game.state.specimen.get_by_id(specimen_id),
                )
                .ui(ui);
            } else {
                SpecimenInfo::new(&mut self.app.textures, None).ui(ui);
            }

            ui.separator();

            SpecimenSelection::new(self.app, self.selection_state).ui(ui);
        });
    }
}
