use crate::app::GameApp;
use crate::components::state::SpecimenSelectionState;
use crate::components::{Component, SpecimenOverview};
use crate::windows::ViewWindow;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct DebugSpecimenWindowState {
    pub is_open: bool,
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
        SpecimenOverview::new(self.app, &mut self.state.selection_state).ui(ui);
    }
}
