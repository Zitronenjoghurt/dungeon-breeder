use crate::app::GameApp;
use crate::components::state::SpecimenSelectionState;
use crate::components::{Component, SpecimenOverview};
use crate::data::tip::Tip;
use crate::windows::ViewWindow;
use dungeon_breeder_core::types::flag::GameFlag;
use egui::{Context, Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct SpecimenWindowState {
    pub is_open: bool,
    pub selection_state: SpecimenSelectionState,
    #[serde(skip, default)]
    pub sort_ready: bool,
    #[serde(skip, default)]
    pub already_opened: bool,
}

pub struct SpecimenWindow<'a> {
    app: &'a mut GameApp,
    state: &'a mut SpecimenWindowState,
}

impl<'a> SpecimenWindow<'a> {
    pub fn new(app: &'a mut GameApp, state: &'a mut SpecimenWindowState) -> Self {
        Self { app, state }
    }
}

impl ViewWindow for SpecimenWindow<'_> {
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
        self.state.is_open = open;
    }

    fn before_close(&mut self, _ctx: &Context) {
        self.state.sort_ready = false;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        if !self.state.already_opened {
            self.state.already_opened = true;
            self.app
                .game
                .actions
                .set_flag(GameFlag::HasClickedSpecimenOverview, true);
            self.app.tips.show_tip(Tip::SpecimenProficiency);
        }

        if !self.state.sort_ready {
            self.state.selection_state.sort_dirty();
            self.state.sort_ready = true;
        }

        SpecimenOverview::new(self.app, &mut self.state.selection_state).ui(ui);
    }
}
