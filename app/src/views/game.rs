use crate::components::*;
use crate::state::AppState;
use crate::views::View;
use crate::windows::debug_window::{DebugWindow, DebugWindowState};
use crate::windows::settings::SettingsWindow;
use crate::windows::ViewWindow;
use egui::{CentralPanel, Context, TopBottomPanel};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct GameView {
    settings_window_open: bool,
    debug_window_state: DebugWindowState,
}

impl View for GameView {
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        SettingsWindow::new(&mut self.settings_window_open, state.settings_mut()).show(ctx);
        DebugWindow::new(state.game(), &mut self.debug_window_state).show(ctx);

        TopBottomPanel::top("game_tab_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ToggleButton::new(&mut self.settings_window_open, " 🛠 ").ui(ui);
                ToggleButton::new(&mut self.debug_window_state.is_open, " 🐛 ").ui(ui);

                ui.separator();

                ui.label(format!("{}💰", state.game().state.treasury.coins()))
            });
        });

        CentralPanel::default().show(ctx, |ui| {});
    }
}
