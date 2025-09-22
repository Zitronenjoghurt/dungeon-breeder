use crate::app::bug_report::{BugReport, BugReportMetadata};
use crate::app::snapshot::GameAppSnapshot;
use crate::modals::ModalSystem;
use crate::systems::actions::{AppAction, AppActions};
use crate::systems::bug_report_review::BugReportReviewSystem;
use crate::systems::dialogue::DialogueSystem;
use crate::systems::file_picker::FilePicker;
use crate::systems::settings::SettingsSystem;
use crate::systems::textures::TextureSystem;
use crate::systems::toasts::ToastSystem;
use crate::theme::apply_glomzy_theme;
use crate::views::{View, ViewID, ViewSystem};
use crate::windows::WindowSystem;
use anyhow::Context;
use dungeon_breeder_core::Game;
use egui::FontDefinitions;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

pub mod bug_report;
pub mod runtime_info;
pub mod snapshot;
pub mod system_info;

#[derive(Default, Serialize, Deserialize)]
pub struct GameApp {
    #[serde(skip, default)]
    pub actions: AppActions,
    pub game: Game,
    pub settings: SettingsSystem,
    pub modals: ModalSystem,
    pub views: ViewSystem,
    pub windows: WindowSystem,
    #[serde(skip, default)]
    pub textures: TextureSystem,
    #[serde(skip, default)]
    pub toasts: ToastSystem,
    #[serde(skip, default)]
    pub file_picker: FilePicker,
    #[serde(skip, default)]
    pub bug_report_review: BugReportReviewSystem,
}

impl eframe::App for GameApp {
    #[tracing::instrument(
        target = "app",
        name = "app::update",
        level = "trace"
        skip(self, ctx, _frame),
    )]
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(100));
        self.handle_keyboard_inputs(ctx);
        self.update_game();
        self.update_views(ctx);
        self.update_windows(ctx);
        self.update_modals(ctx);
        self.update_file_picker(ctx);
        self.settings.update(ctx);
        self.toasts.update(ctx);
        self.update_dialogue(ctx);

        for action in self.actions.take_actions() {
            self.handle_app_action(ctx, action);
        }
    }

    #[tracing::instrument(
        target = "app",
        name = "app::save",
        level = "trace"
        skip(self, storage),
    )]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

impl GameApp {
    pub fn new(cc: &eframe::CreationContext) -> anyhow::Result<Self> {
        let mut fonts = FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        cc.egui_ctx.set_fonts(fonts);

        Self::setup_context(&cc.egui_ctx);

        #[cfg(debug_assertions)]
        {
            Ok(cc
                .storage
                .and_then(|storage| eframe::get_value::<Self>(storage, eframe::APP_KEY))
                .unwrap_or_default())
        }

        #[cfg(not(debug_assertions))]
        {
            match cc.storage {
                Some(storage) => match eframe::get_value::<Self>(storage, eframe::APP_KEY) {
                    Some(app) => Ok(app),
                    None => {
                        if storage.get_string(eframe::APP_KEY).is_some() {
                            Err(anyhow::anyhow!(
                                "Failed to deserialize app state - corrupted or incompatible format"
                            ))
                        } else {
                            Ok(Self::default())
                        }
                    }
                },
                None => Ok(Self::default()),
            }
        }
    }

    fn setup_context(ctx: &egui::Context) {
        apply_glomzy_theme(ctx);
    }

    #[tracing::instrument(
        target = "app",
        name = "app::update_game",
        level = "trace"
        skip(self),
    )]
    fn update_game(&mut self) {
        if (self.views.current_view() != ViewID::Game) {
            return;
        }

        let report = self.game.update();
        for error in report.action_report.errors {
            self.toasts.error(error.to_string());
        }

        if report.ticks_elapsed > 10 {
            self.modals
                .offline_progress
                .open(report.progress_report, report.ticks_elapsed);
        }
    }

    #[tracing::instrument(
        target = "app",
        name = "app::update_modals",
        level = "trace"
        skip(self, ctx),
    )]
    fn update_modals(&mut self, ctx: &egui::Context) {
        let mut modal_system = std::mem::take(&mut self.modals);
        modal_system.update(ctx, self);
        self.modals = modal_system;
    }

    #[tracing::instrument(
        target = "app",
        name = "app::views",
        level = "trace"
        skip(self, ctx),
    )]
    fn update_views(&mut self, ctx: &egui::Context) {
        let mut view_system = std::mem::take(&mut self.views);
        view_system.render(ctx, self);
        self.views = view_system;
    }

    #[tracing::instrument(
        target = "app",
        name = "app::update_windows",
        level = "trace"
        skip(self, ctx),
    )]
    fn update_windows(&mut self, ctx: &egui::Context) {
        let mut window_system = std::mem::take(&mut self.windows);
        window_system.update(ctx, self);
        self.windows = window_system;
    }

    #[tracing::instrument(
        target = "app",
        name = "app::update_file_picker",
        level = "trace"
        skip(self, ctx),
    )]
    fn update_file_picker(&mut self, ctx: &egui::Context) {
        let mut file_picker = std::mem::take(&mut self.file_picker);
        file_picker.update(ctx, self);
        self.file_picker = file_picker;
    }

    #[tracing::instrument(
        target = "app",
        name = "app::update_dialogue",
        level = "trace"
        skip(self, ctx),
    )]
    fn update_dialogue(&mut self, ctx: &egui::Context) {
        DialogueSystem::update(ctx, self);
    }

    #[tracing::instrument(
        target = "app",
        name = "app::handle_keyboard_inputs",
        level = "trace"
        skip(self, ctx),
    )]
    fn handle_keyboard_inputs(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.key_pressed(egui::Key::F3)) {
            self.windows.debug.is_open = !self.windows.debug.is_open;
        }
    }

    #[tracing::instrument(
        target = "app",
        name = "app::handle_app_action",
        level = "trace"
        skip(self, ctx),
    )]
    fn handle_app_action(&mut self, ctx: &egui::Context, action: AppAction) {
        let result = match action {
            AppAction::SaveAppSnapshot(path) => self.handle_save_app_snapshot(path, ctx),
            AppAction::RestoreAppSnapshot(path) => self.handle_restore_app_snapshot(path, ctx),
            AppAction::DumpAppJSON(path) => self.handle_dump_app_json(path),
            AppAction::CreateBugReport { path, metadata } => {
                self.handle_create_bug_report(path, metadata, ctx)
            }
            AppAction::ReviewBugReport(path) => self.handle_review_bug_report(path),
            AppAction::RestoreBugReport => self.handle_restore_bug_report(ctx),
            AppAction::SwitchView(view) => self.handle_switch_view(view),
        };

        if let Err(error) = result {
            self.toasts.error(error.to_string());
        }
    }
}

// App action handlers
impl GameApp {
    fn handle_save_app_snapshot(&self, path: PathBuf, ctx: &egui::Context) -> anyhow::Result<()> {
        let snapshot = GameAppSnapshot::create(self, ctx)?;
        let bytes = snapshot.export_rmp()?;
        std::fs::write(path, bytes)?;
        Ok(())
    }

    fn handle_restore_app_snapshot(
        &mut self,
        path: PathBuf,
        ctx: &egui::Context,
    ) -> anyhow::Result<()> {
        let bytes = std::fs::read(path)?;
        let snapshot = GameAppSnapshot::import_rmp(&bytes)?;
        snapshot.restore(self, ctx)?;
        Ok(())
    }

    fn handle_dump_app_json(&self, path: PathBuf) -> anyhow::Result<()> {
        let string = serde_json::to_string_pretty(&self)?;
        std::fs::write(path, string)?;
        Ok(())
    }

    fn handle_create_bug_report(
        &self,
        path: PathBuf,
        meta: BugReportMetadata,
        ctx: &egui::Context,
    ) -> anyhow::Result<()> {
        let report = BugReport::create(meta, self, ctx)?;
        let bytes = report.export_rmp()?;
        std::fs::write(path, bytes)?;
        Ok(())
    }

    fn handle_review_bug_report(&mut self, path: PathBuf) -> anyhow::Result<()> {
        let bytes = std::fs::read(path)?;
        let bug_report = BugReport::import_rmp(&bytes)?;
        self.bug_report_review.set_bug_report(bug_report);
        Ok(())
    }

    fn handle_restore_bug_report(&mut self, ctx: &egui::Context) -> anyhow::Result<()> {
        let bug_report = self
            .bug_report_review
            .take_bug_report()
            .context("No bug report loaded for review")?;
        bug_report.snapshot.restore(self, ctx)?;
        Ok(())
    }

    fn handle_switch_view(&mut self, view_id: ViewID) -> anyhow::Result<()> {
        self.views.switch_view(view_id);
        Ok(())
    }
}
