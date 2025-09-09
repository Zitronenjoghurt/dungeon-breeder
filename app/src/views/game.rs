use crate::app::GameApp;
use crate::components::*;
use crate::views::View;
use crate::windows::dungeon::DungeonWindow;
use crate::windows::items::ItemsWindow;
use crate::windows::specimen::{SpecimenWindow, SpecimenWindowState};
use crate::windows::statistics::StatisticsWindow;
use crate::windows::ViewWindow;
use egui::{CentralPanel, Context, TopBottomPanel};
use egui_phosphor::regular;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct GameView {
    dungeon_window_open: bool,
    items_window_open: bool,
    statistics_window_open: bool,
    specimen_window: SpecimenWindowState,
}

impl View for GameView {
    fn render(&mut self, ctx: &Context, app: &mut GameApp) {
        DungeonWindow::new(&mut app.modals, &app.game, &mut self.dungeon_window_open).show(ctx);
        ItemsWindow::new(&app.game, &mut self.items_window_open).show(ctx);
        SpecimenWindow::new(app, &mut self.specimen_window).show(ctx);
        StatisticsWindow::new(&app.game.state.statistics, &mut self.statistics_window_open)
            .show(ctx);

        TopBottomPanel::top("game_tab_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ToggleButton::new(&mut app.windows.settings_open, regular::GEAR)
                    .tooltip("Settings")
                    .ui(ui);

                ToggleButton::new(
                    &mut self.statistics_window_open,
                    regular::PROJECTOR_SCREEN_CHART,
                )
                .tooltip("Statistics")
                .ui(ui);

                ToggleButton::new(&mut app.windows.debug.is_open, regular::BUG)
                    .tooltip("Debug")
                    .ui(ui);

                ui.separator();

                ToggleButton::new(&mut self.specimen_window.is_open, regular::WAREHOUSE)
                    .tooltip("Specimen")
                    .ui(ui);

                ToggleButton::new(&mut self.items_window_open, regular::TREASURE_CHEST)
                    .tooltip("Items")
                    .ui(ui);

                ToggleButton::new(&mut self.dungeon_window_open, regular::SWORD)
                    .tooltip("Dungeon")
                    .ui(ui);

                ToggleButton::new(&mut app.windows.breeding.is_open, regular::HEART)
                    .tooltip("Breeding")
                    .ui(ui);

                ToggleButton::new(&mut app.windows.fusion.is_open, regular::ARROWS_MERGE)
                    .tooltip("Fusion")
                    .ui(ui);

                ui.separator();

                ui.label(format!(
                    "{} {}",
                    app.game.state.treasury.coins(),
                    regular::COINS
                ))
            });
        });

        CentralPanel::default().show(ctx, |ui| {});
    }
}
