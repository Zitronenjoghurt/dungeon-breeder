use crate::app::GameApp;
use crate::windows::ViewWindow;
use dungeon_breeder_core::data::creature::id::CreatureID;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct DebugWindowState {
    pub is_open: bool,
}

pub struct DebugWindow<'a> {
    app: &'a mut GameApp,
    state: &'a mut DebugWindowState,
}

impl<'a> DebugWindow<'a> {
    pub fn new(app: &'a mut GameApp, state: &'a mut DebugWindowState) -> Self {
        Self { app, state }
    }
}

impl ViewWindow for DebugWindow<'_> {
    fn id(&self) -> Id {
        Id::new("debug_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Debug"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("Random Gonk").clicked() {
                self.app.game.actions.random_specimen(CreatureID::Gonk);
            }

            if ui.button("Eat the Rich").clicked() {
                self.app.game.actions.add_coins(1_000_000);
            }
        });
    }
}
