use crate::components::spawn::{SpecimenSpawnComponent, SpecimenSpawnComponentState};
use crate::components::Component;
use crate::windows::ViewWindow;
use dungeon_breeder_core::Game;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SpecimenSpawnDebugWindowState {
    pub is_open: bool,
    specimen_spawn_component_state: SpecimenSpawnComponentState,
}

pub struct SpecimenSpawnDebugWindow<'a> {
    state: &'a mut SpecimenSpawnDebugWindowState,
    game: &'a Game,
}

impl<'a> SpecimenSpawnDebugWindow<'a> {
    pub fn new(state: &'a mut SpecimenSpawnDebugWindowState, game: &'a Game) -> Self {
        Self { state, game }
    }
}

impl ViewWindow for SpecimenSpawnDebugWindow<'_> {
    fn id(&self) -> Id {
        Id::new("specimen_spawn_debug_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Specimen Spawn Debug"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        SpecimenSpawnComponent::new(&mut self.state.specimen_spawn_component_state, self.game)
            .ui(ui);
    }
}
