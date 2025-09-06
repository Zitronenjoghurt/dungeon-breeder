use crate::app::GameApp;
use crate::components::state::SpecimenSelectionState;
use crate::components::{Component, SpecimenModalSelection, SpecimenSelection};
use crate::windows::ViewWindow;
use dungeon_breeder_core::state::specimen::SpecimenId;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct DebugSpecimenWindowState {
    pub is_open: bool,
    pub selected_specimen_id_a: Option<SpecimenId>,
    pub selected_specimen_id_b: Option<SpecimenId>,
    pub selection_state: SpecimenSelectionState,
}

pub struct DebugSpecimenWindow<'a> {
    app: &'a mut GameApp,
    state: &'a mut DebugSpecimenWindowState,
}

impl<'a> DebugSpecimenWindow<'a> {
    pub fn new(app: &'a mut GameApp, state: &'a mut DebugSpecimenWindowState) -> Self {
        Self { app, state }
    }
}

impl ViewWindow for DebugSpecimenWindow<'_> {
    fn id(&self) -> Id {
        Id::new("debug_specimen_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Specimen"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        let just_opened = !self.state.is_open && open;
        self.state.is_open = open;

        if just_opened {
            self.state.selection_state.sort_dirty();
        }
    }

    fn render_content(&mut self, ui: &mut Ui) {
        SpecimenModalSelection::new(
            &mut self.app.modals,
            &self.app.game.state.specimen,
            self.state.selected_specimen_id_a,
            move |specimen_id, app| {
                app.windows.debug.specimen_window.selected_specimen_id_a = specimen_id
            },
        )
        .exclude_on_breeding_cooldown(true)
        .ui(ui);

        SpecimenModalSelection::new(
            &mut self.app.modals,
            &self.app.game.state.specimen,
            self.state.selected_specimen_id_b,
            move |specimen_id, app| {
                app.windows.debug.specimen_window.selected_specimen_id_b = specimen_id
            },
        )
        .exclude_on_breeding_cooldown(true)
        .ui(ui);

        if ui.button("Breed").clicked()
            && let Some(specimen_id_a) = self.state.selected_specimen_id_a
            && let Some(specimen_id_b) = self.state.selected_specimen_id_b
        {
            self.app.game.actions.breed(specimen_id_a, specimen_id_b);
        }

        if ui.button("Fuse").clicked()
            && let Some(specimen_id_a) = self.state.selected_specimen_id_a
            && let Some(specimen_id_b) = self.state.selected_specimen_id_b
        {
            self.app.game.actions.fuse(specimen_id_a, specimen_id_b);
        }

        SpecimenSelection::new(self.app, &mut self.state.selection_state)
            .selection_enabled(false)
            .ui(ui);
    }
}
