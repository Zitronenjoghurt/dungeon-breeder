use crate::app::GameApp;
use crate::components::compendium::{CompendiumComponent, CompendiumComponentState};
use crate::components::Component;
use crate::windows::ViewWindow;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CompendiumWindowState {
    pub is_open: bool,
    compendium_component_state: CompendiumComponentState,
}

pub struct CompendiumWindow<'a> {
    app: &'a mut GameApp,
    state: &'a mut CompendiumWindowState,
}

impl<'a> CompendiumWindow<'a> {
    pub fn new(app: &'a mut GameApp, state: &'a mut CompendiumWindowState) -> Self {
        Self { app, state }
    }
}

impl ViewWindow for CompendiumWindow<'_> {
    fn id(&self) -> Id {
        Id::new("compendium_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Compendium"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        CompendiumComponent::new(
            &mut self.app.textures,
            &self.app.game,
            &mut self.state.compendium_component_state,
        )
        .ui(ui);
    }
}
