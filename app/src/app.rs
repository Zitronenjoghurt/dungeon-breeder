use crate::app::bug_report::{BugReport, BugReportMetadata};
use crate::app::persistence::migration::PersistenceMetadata;
use crate::app::snapshot::GameAppSnapshot;
use crate::app_save_file_path;
use crate::modals::ModalSystem;
use crate::systems::actions::{AppAction, AppActions};
use crate::systems::bug_report_review::BugReportReviewSystem;
use crate::systems::debug_stats::DebugStatsSystem;
use crate::systems::dialogue::DialogueSystem;
use crate::systems::file_picker::FilePicker;
use crate::systems::settings::SettingsSystem;
use crate::systems::textures::TextureSystem;
use crate::systems::tips::TipsSystem;
use crate::systems::toasts::ToastSystem;
use crate::theme::apply_glomzy_theme;
use crate::types::font::CustomFont;
use crate::views::{View, ViewID, ViewSystem};
use crate::windows::WindowSystem;
use anyhow::Context;
use dungeon_breeder_core::feedback::notification::{
    GameFeedbackNotification, GameFeedbackNotificationType,
};
use dungeon_breeder_core::feedback::GameFeedback;
use dungeon_breeder_core::types::flag::GameFlag;
use dungeon_breeder_core::Game;
use egui::FontDefinitions;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

pub mod bug_report;
pub mod performance_info;
mod persistence;
pub mod runtime_info;
pub mod snapshot;
pub mod system_info;

#[derive(Default, Serialize, Deserialize)]
pub struct GameApp {
    #[serde(skip, default)]
    pub actions: AppActions,
    pub game: Game,
    pub settings: SettingsSystem,
    pub tips: TipsSystem,
    pub modals: ModalSystem,
    pub views: ViewSystem,
    pub windows: WindowSystem,
    pub persistence_metadata: PersistenceMetadata,
    #[serde(skip, default)]
    pub textures: TextureSystem,
    #[serde(skip, default)]
    pub toasts: ToastSystem,
    #[serde(skip, default)]
    pub file_picker: FilePicker,
    #[serde(skip, default)]
    pub bug_report_review: BugReportReviewSystem,
    #[serde(skip, default)]
    pub debug_stats: DebugStatsSystem,
}

impl eframe::App for GameApp {
    #[tracing::instrument(
        target = "app",
        name = "app::update",
        level = "trace"
        skip(self, ctx, _frame),
    )]
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = std::time::Instant::now();

        ctx.request_repaint_after(Duration::from_millis(100));
        self.handle_keyboard_inputs(ctx);
        self.update_game(ctx);
        self.update_views(ctx);
        self.update_windows(ctx);
        self.update_modals(ctx);
        self.update_file_picker(ctx);
        self.settings.update(ctx);
        self.toasts.update(ctx);
        self.update_dialogue(ctx);
        self.update_debug_stats();

        for action in self.actions.take_actions() {
            self.handle_app_action(ctx, action);
        }

        let elapsed = now.elapsed();
        self.debug_stats.process_app_update_duration(elapsed);
    }

    #[tracing::instrument(
        target = "app",
        name = "app::save",
        level = "trace"
        skip(self, _storage),
    )]
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        let result = self.persistence_metadata.save();
        if let Err(error) = result {
            self.toasts.error(error.to_string());
        }

        let result = self.save_to_file(&app_save_file_path());
        if let Err(error) = result {
            self.toasts.error(error.to_string());
        }
    }
}

impl GameApp {
    pub fn new(cc: &eframe::CreationContext) -> anyhow::Result<Self> {
        Self::setup_context(&cc.egui_ctx);

        let persistence_meta = PersistenceMetadata::load()?;
        persistence_meta.load_app()
    }

    fn setup_context(ctx: &egui::Context) {
        apply_glomzy_theme(ctx);
        Self::setup_fonts(ctx);
    }

    fn setup_fonts(ctx: &egui::Context) {
        let mut fonts = FontDefinitions::default();
        CustomFont::load_all(&mut fonts);
        CustomFont::default().set_as_default(&mut fonts);
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        ctx.set_fonts(fonts);
    }

    #[tracing::instrument(
        target = "app",
        name = "app::update_game",
        level = "trace"
        skip(self),
    )]
    fn update_game(&mut self, ctx: &egui::Context) {
        if (self.views.current_view() != ViewID::Game) {
            return;
        }

        let report = self.game.update();
        self.debug_stats.process_report(&report);

        for error in report.action_report.errors {
            self.toasts.error(error.to_string());
        }

        for feedback in report.feedback {
            self.handle_game_feedback(ctx, feedback);
        }

        if report.ticks_elapsed > 10
            && self
                .game
                .state
                .flags
                .get(GameFlag::OfflineProgressionReportEnabled)
        {
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
        name = "app::update_debug_stats",
        level = "trace"
        skip(self),
    )]
    fn update_debug_stats(&mut self) {
        self.debug_stats.update();
    }

    #[tracing::instrument(
        target = "app",
        name = "app::handle_keyboard_inputs",
        level = "trace"
        skip(self, ctx),
    )]
    fn handle_keyboard_inputs(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            if i.key_pressed(egui::Key::F3) {
                if i.key_down(egui::Key::D) {
                    self.game.actions.debug_dialogue_bg_interactive();
                } else {
                    self.windows.debug.is_open = !self.windows.debug.is_open;
                }
            }
        });
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

    fn handle_game_feedback(&mut self, ctx: &egui::Context, feedback: GameFeedback) {
        match feedback {
            GameFeedback::CloseApp => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
            GameFeedback::Notification(notification) => {
                self.handle_feedback_notification(notification);
            }
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

// App game feedback handlers
impl GameApp {
    fn handle_feedback_notification(&mut self, notification: GameFeedbackNotification) {
        match notification.notification_type {
            GameFeedbackNotificationType::Info => self.toasts.info(notification.message),
            GameFeedbackNotificationType::Success => self.toasts.success(notification.message),
            GameFeedbackNotificationType::Error => self.toasts.error(notification.message),
        }
    }
}
