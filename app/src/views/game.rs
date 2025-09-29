use crate::app::GameApp;
use crate::components::game_menu_button::GameMenuButton;
use crate::components::*;
use crate::utils::formatting::format_number;
use crate::views::{View, ViewID};
use crate::windows::compendium::{CompendiumWindow, CompendiumWindowState};
use crate::windows::dungeon::{DungeonWindow, DungeonWindowState};
use crate::windows::items::ItemsWindow;
use crate::windows::specimen::{SpecimenWindow, SpecimenWindowState};
use crate::windows::statistics::StatisticsWindow;
use crate::windows::summon::SummonWindow;
use crate::windows::tips::TipsWindow;
use crate::windows::ViewWindow;
use dungeon_breeder_core::types::flag::GameFlag;
use egui::{CentralPanel, Context, TopBottomPanel};
use egui_phosphor::regular;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct GameView {
    items_window_open: bool,
    statistics_window_open: bool,
    summoning_window_open: bool,
    compendium_window: CompendiumWindowState,
    dungeon_window: DungeonWindowState,
    specimen_window: SpecimenWindowState,
}

impl GameView {
    pub fn show_top_bar(&mut self, ctx: &Context, app: &mut GameApp) {
        TopBottomPanel::top("game_tab_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if app.game.state.flags.get(GameFlag::TutorialComplete)
                    && ui
                        .selectable_label(false, regular::HOUSE)
                        .on_hover_text("Main Menu")
                        .clicked()
                {
                    app.actions.switch_view(ViewID::MainMenu);
                }

                ToggleButton::new(&mut app.windows.bug_report.is_open, regular::BUG)
                    .tooltip("Bug Report")
                    .ui(ui);

                ToggleButton::new(&mut app.windows.settings_open, regular::GEAR)
                    .tooltip("Settings")
                    .ui(ui);

                GameMenuButton::new(&mut app.tips.is_window_open)
                    .label(regular::QUESTION)
                    .tooltip("Hints")
                    .ui(ui);

                GameMenuButton::new(&mut self.statistics_window_open)
                    .label(regular::PROJECTOR_SCREEN_CHART)
                    .unlocked(app.game.state.flags.get(GameFlag::UnlockedStatistics))
                    .tooltip("Statistics")
                    .ui(ui);

                ui.separator();

                GameMenuButton::new(&mut self.specimen_window.is_open)
                    .label(regular::ANDROID_LOGO)
                    .unlocked(app.game.state.flags.get(GameFlag::UnlockedSpecimenOverview))
                    .tooltip("Specimen")
                    .ui(ui);

                GameMenuButton::new(&mut app.windows.breeding.is_open)
                    .label(regular::HEART)
                    .unlocked(app.game.state.flags.get(GameFlag::UnlockedBreeding))
                    .tooltip("Breeding")
                    .ui(ui);

                GameMenuButton::new(&mut app.windows.fusion.is_open)
                    .label(regular::ARROWS_MERGE)
                    .unlocked(app.game.state.flags.get(GameFlag::UnlockedFusion))
                    .tooltip("Fusion")
                    .ui(ui);

                GameMenuButton::new(&mut self.dungeon_window.is_open)
                    .label(regular::SWORD)
                    .unlocked(app.game.state.flags.get(GameFlag::UnlockedDungeon))
                    .tooltip("Dungeon")
                    .ui(ui);

                GameMenuButton::new(&mut self.items_window_open)
                    .label(regular::TREASURE_CHEST)
                    .unlocked(app.game.state.flags.get(GameFlag::UnlockedItems))
                    .tooltip("Items")
                    .ui(ui);

                GameMenuButton::new(&mut self.summoning_window_open)
                    .label(regular::FLAME)
                    .unlocked(app.game.state.flags.get(GameFlag::UnlockedSummoning))
                    .tooltip("Summoning")
                    .ui(ui);

                GameMenuButton::new(&mut self.compendium_window.is_open)
                    .label(regular::BOOK)
                    .unlocked(app.game.state.flags.get(GameFlag::UnlockedCompendium))
                    .tooltip("Compendium")
                    .ui(ui);

                ui.separator();

                ui.label(format!(
                    "{} {}",
                    format_number(app.game.state.treasury.coins()),
                    regular::COINS
                ))
            });
        });
    }
}

impl View for GameView {
    fn render(&mut self, ctx: &Context, app: &mut GameApp) {
        CompendiumWindow::new(app, &mut self.compendium_window).show(ctx);
        DungeonWindow::new(
            &mut app.modals,
            &mut app.tips,
            &app.game,
            &mut self.dungeon_window,
        )
        .show(ctx);
        ItemsWindow::new(&app.game, &mut self.items_window_open).show(ctx);
        SpecimenWindow::new(app, &mut self.specimen_window).show(ctx);
        StatisticsWindow::new(&app.game.state.statistics, &mut self.statistics_window_open)
            .show(ctx);
        SummonWindow::new(app, &mut self.summoning_window_open).show(ctx);
        TipsWindow::new(app).show(ctx);

        if app.game.state.flags.get(GameFlag::UnlockedTopBar) {
            self.show_top_bar(ctx, app);
        }

        CentralPanel::default().show(ctx, |ui| {});
    }
}
