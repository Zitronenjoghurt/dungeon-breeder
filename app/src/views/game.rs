use crate::components::specimen::table::SpecimenTable;
use crate::components::*;
use crate::state::AppState;
use crate::views::View;
use crate::windows::settings::SettingsWindow;
use egui::{CentralPanel, Context, TopBottomPanel};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct GameView {
    settings_window: SettingsWindow,
}

impl View for GameView {
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        WindowRenderer::new(ctx, state)
            .window(&mut self.settings_window)
            .render();

        TopBottomPanel::top("game_tab_bar").show(ctx, |ui| {
            WindowButton::new(&mut self.settings_window, " 🛠 ").ui(ui);
        });

        CentralPanel::default().show(ctx, |ui| {
            SpecimenTable::new(state.game(), &state.game().state.specimen).ui(ui);
        });
    }
}
