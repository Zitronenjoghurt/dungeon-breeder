use crate::app::GameApp;
use crate::components::{Component, EnumSelect};
use crate::windows::ViewWindow;
use dungeon_breeder_core::types::flag::GameFlag;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FlagsDebugWindowState {
    pub is_open: bool,
    selected_flag: GameFlag,
}

pub struct FlagsDebugWindow<'a> {
    pub app: &'a mut GameApp,
    pub state: &'a mut FlagsDebugWindowState,
}

impl<'a> FlagsDebugWindow<'a> {
    pub fn new(app: &'a mut GameApp, state: &'a mut FlagsDebugWindowState) -> Self {
        Self { app, state }
    }
}

impl ViewWindow for FlagsDebugWindow<'_> {
    fn id(&self) -> Id {
        Id::new("debug_flags_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Flags Debug"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            EnumSelect::new(
                &mut self.state.selected_flag,
                "debug_flags_window_flag_id_select",
            )
            .ui(ui);

            let mut value = self.app.game.state.flags.get(self.state.selected_flag);
            let value_before = value;
            ui.checkbox(&mut value, "Active");

            if value != value_before {
                self.app
                    .game
                    .actions
                    .set_flag(self.state.selected_flag, value);
            }
        });
    }
}
