use crate::app::GameApp;
use crate::components::{Component, ToggleButton};
use crate::modals::confirm::ConfirmModalOptions;
use crate::systems::file_picker::FilePickerConfig;
use crate::windows::debug_windows::bug_report::{BugReportDebugWindow, BugReportDebugWindowState};
use crate::windows::debug_windows::dialogue::{DialogueDebugWindow, DialogueDebugWindowState};
use crate::windows::debug_windows::flags::{FlagsDebugWindow, FlagsDebugWindowState};
use crate::windows::debug_windows::font::{FontDebugWindow, FontDebugWindowState};
use crate::windows::debug_windows::specimen_spawn::{
    SpecimenSpawnDebugWindow, SpecimenSpawnDebugWindowState,
};
use crate::windows::debug_windows::stats::DebugStatsWindow;
use crate::windows::debug_windows::time::TimeDebugWindow;
use crate::windows::debug_windows::tips::{TipsDebugWindow, TipsDebugWindowState};
use crate::windows::{ViewWindow, WindowSystem};
use dungeon_breeder_core::data::creature::id::CreatureID;
use egui::{Id, Ui, WidgetText};
use egui_phosphor::regular;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct DebugWindowState {
    pub is_open: bool,
    pub stats_open: bool,
    pub time_open: bool,
    pub bug_report: BugReportDebugWindowState,
    pub dialogue: DialogueDebugWindowState,
    pub flags: FlagsDebugWindowState,
    pub fonts: FontDebugWindowState,
    pub specimen_spawn: SpecimenSpawnDebugWindowState,
    pub tips: TipsDebugWindowState,
}

pub struct DebugWindow<'a> {
    app: &'a mut GameApp,
    windows: &'a mut WindowSystem,
    state: &'a mut DebugWindowState,
}

impl<'a> DebugWindow<'a> {
    pub fn new(
        app: &'a mut GameApp,
        windows: &'a mut WindowSystem,
        state: &'a mut DebugWindowState,
    ) -> Self {
        Self {
            app,
            windows,
            state,
        }
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
        BugReportDebugWindow::new(self.app, &mut self.state.bug_report).show(ui.ctx());
        DebugStatsWindow::new(self.app, &mut self.state.stats_open).show(ui.ctx());
        DialogueDebugWindow::new(self.app, &mut self.state.dialogue).show(ui.ctx());
        FlagsDebugWindow::new(self.app, &mut self.state.flags).show(ui.ctx());
        FontDebugWindow::new(self.app, &mut self.state.fonts).show(ui.ctx());
        SpecimenSpawnDebugWindow::new(&mut self.state.specimen_spawn, &self.app.game)
            .show(ui.ctx());
        TimeDebugWindow::new(&self.app.game, &mut self.state.time_open).show(ui.ctx());
        TipsDebugWindow::new(&mut self.state.tips, self.app).show(ui.ctx());

        ui.horizontal(|ui| {
            ToggleButton::new(&mut self.state.stats_open, regular::CHART_BAR).ui(ui);
            ToggleButton::new(&mut self.state.bug_report.is_open, regular::BUG).ui(ui);
            ToggleButton::new(&mut self.state.specimen_spawn.is_open, regular::ALIEN).ui(ui);
            ToggleButton::new(&mut self.state.dialogue.is_open, regular::CHAT).ui(ui);
            ToggleButton::new(&mut self.state.flags.is_open, regular::FLAG).ui(ui);
            ToggleButton::new(&mut self.state.fonts.is_open, regular::TEXT_A_UNDERLINE).ui(ui);
            ToggleButton::new(&mut self.state.tips.is_open, regular::QUESTION).ui(ui);
            ToggleButton::new(&mut self.state.time_open, regular::CLOCK).ui(ui);
        });

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Export Snapshot").clicked() {
                self.app.file_picker.open(
                    FilePickerConfig::save().default_file_name("snapshot.dbsp"),
                    |app, paths| {
                        let Some(path) = paths.first() else {
                            return;
                        };
                        app.actions.save_app_snapshot(path);
                    },
                )
            }

            if ui.button("Restore Snapshot").clicked() {
                self.app
                    .file_picker
                    .open(FilePickerConfig::pick_single(), |app, paths| {
                        let Some(path) = paths.first() else {
                            return;
                        };
                        app.actions.restore_app_snapshot(path);
                    })
            }

            if ui.button("Dump JSON").clicked() {
                self.app.file_picker.open(
                    FilePickerConfig::save().default_file_name("app_dump.json"),
                    |app, paths| {
                        let Some(path) = paths.first() else {
                            return;
                        };
                        app.actions.dump_app_json(path);
                    },
                )
            }
        });

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Random Gonk").clicked() {
                self.app.game.actions.random_specimen(CreatureID::Gonk);
            }

            if ui.button("Random Specimen").clicked() {
                self.app.game.actions.random_specimen(CreatureID::random());
            }

            if ui.button("Eat the Rich").clicked() {
                self.app.game.actions.add_coins(1_000_000);
            }
        });

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Reset Game").clicked() {
                let modal_options = ConfirmModalOptions::new(
                    "Delete all game data?",
                    "Once deleted, the game state cannot be restored.",
                )
                .yes("Delete")
                .no("Cancel");
                self.app
                    .modals
                    .confirm
                    .open(modal_options, |app| app.game.actions.reset_game_state());
            }
        });
    }
}
