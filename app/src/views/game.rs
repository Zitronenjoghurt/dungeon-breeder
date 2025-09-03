use crate::app::GameApp;
use crate::components::*;
use crate::views::View;
use egui::{CentralPanel, Context, TopBottomPanel};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct GameView {}

impl View for GameView {
    fn render(&mut self, ctx: &Context, app: &mut GameApp) {
        TopBottomPanel::top("game_tab_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ToggleButton::new(&mut app.windows.settings_open, " 🛠 ").ui(ui);
                ToggleButton::new(&mut app.windows.debug.is_open, " 🐛 ").ui(ui);

                ui.separator();

                ui.label(format!("{}💰", app.game.state.treasury.coins()))
            });
        });

        CentralPanel::default().show(ctx, |ui| {});
    }
}
