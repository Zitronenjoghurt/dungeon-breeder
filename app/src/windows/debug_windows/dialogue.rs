use crate::app::GameApp;
use crate::components::{Component, EnumSelect};
use crate::windows::ViewWindow;
use dungeon_breeder_core::data::dialogue::id::DialogueID;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DialogueDebugWindowState {
    pub is_open: bool,
    pub dialogue_id: DialogueID,
}

pub struct DialogueDebugWindow<'a> {
    app: &'a mut GameApp,
    state: &'a mut DialogueDebugWindowState,
}

impl<'a> DialogueDebugWindow<'a> {
    pub fn new(app: &'a mut GameApp, state: &'a mut DialogueDebugWindowState) -> Self {
        Self { app, state }
    }
}

impl ViewWindow for DialogueDebugWindow<'_> {
    fn id(&self) -> Id {
        Id::new("dialogue_debug_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Dialogue Debug"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        EnumSelect::new(
            &mut self.state.dialogue_id,
            "dialogue_debug_window_dialogue_id_select",
        )
        .label("Dialogue ID")
        .ui(ui);

        if ui.button("Trigger").clicked() {
            self.app
                .game
                .actions
                .trigger_dialogue(self.state.dialogue_id);
        }
    }
}
