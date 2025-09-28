use crate::app::GameApp;
use crate::components::{Component, EnumSelect};
use crate::data::tip::Tip;
use crate::windows::ViewWindow;
use egui::{Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TipsDebugWindowState {
    pub is_open: bool,
    selected_tip: Tip,
}

pub struct TipsDebugWindow<'a> {
    pub state: &'a mut TipsDebugWindowState,
    pub app: &'a mut GameApp,
}

impl<'a> TipsDebugWindow<'a> {
    pub fn new(state: &'a mut TipsDebugWindowState, app: &'a mut GameApp) -> Self {
        Self { state, app }
    }
}

impl ViewWindow for TipsDebugWindow<'_> {
    fn id(&self) -> egui::Id {
        egui::Id::new("tips_debug_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Tips Debug"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        let selected_tip = self.state.selected_tip;

        ui.vertical(|ui| {
            EnumSelect::new(
                &mut self.state.selected_tip,
                "tips_debug_window_tips_select",
            )
            .ui(ui);
            let mut show = self.app.tips.show.get(selected_tip as usize);
            ui.checkbox(&mut show, "Show");
            self.app.tips.show.set_value(selected_tip as usize, show);

            let mut read = self.app.tips.read.get(selected_tip as usize);
            ui.checkbox(&mut read, "Read");
            self.app.tips.read.set_value(selected_tip as usize, read);
        });
    }
}
