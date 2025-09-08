use crate::app::GameApp;
use crate::windows::ViewWindow;
use dungeon_breeder_core::state::specimen::SpecimenId;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FusionWindowState {
    pub is_open: bool,
    pub selected_specimen_1: Option<SpecimenId>,
    pub selected_specimen_2: Option<SpecimenId>,
}

pub struct FusionWindow<'a> {
    pub app: &'a mut GameApp,
    pub state: &'a mut FusionWindowState,
}

impl<'a> FusionWindow<'a> {
    pub fn new(app: &'a mut GameApp, state: &'a mut FusionWindowState) -> Self {
        Self { app, state }
    }
}

impl ViewWindow for FusionWindow<'_> {
    fn id(&self) -> Id {
        Id::new("fusion_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Fusion"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {}
}
