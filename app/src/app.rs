use crate::app::snapshot::GameAppSnapshot;
use crate::modals::ModalSystem;
use crate::systems::actions::{AppAction, AppActions};
use crate::systems::file_picker::FilePicker;
use crate::systems::settings::SettingsSystem;
use crate::systems::textures::TextureSystem;
use crate::systems::toasts::ToastSystem;
use crate::theme::apply_glomzy_theme;
use crate::views::{View, ViewSystem};
use crate::windows::WindowSystem;
use anyhow::anyhow;
use dungeon_breeder_core::Game;
use egui::FontDefinitions;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

mod snapshot;

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
}

impl eframe::App for GameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(500));
        self.handle_keyboard_inputs(ctx);
        self.update_game();
        self.update_views(ctx);
        self.update_windows(ctx);
        self.update_modals(ctx);
        self.update_file_picker(ctx);
        self.settings.update(ctx);
        self.toasts.update(ctx);

        for action in self.actions.take_actions() {
            self.handle_app_action(ctx, action);
        }
    }

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

        match cc.storage {
            Some(storage) => match eframe::get_value::<Self>(storage, eframe::APP_KEY) {
                Some(app) => Ok(app),
                None => {
                    if storage.get_string(eframe::APP_KEY).is_some() {
                        Err(anyhow!(
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

    fn setup_context(ctx: &egui::Context) {
        apply_glomzy_theme(ctx);
    }

    fn update_game(&mut self) {
        let report = self.game.update();
        for error in report.action_report.errors {
            self.toasts.error(error.to_string());
        }
    }

    fn update_modals(&mut self, ctx: &egui::Context) {
        let mut modal_system = std::mem::take(&mut self.modals);
        modal_system.update(ctx, self);
        self.modals = modal_system;
    }

    fn update_views(&mut self, ctx: &egui::Context) {
        let mut view_system = std::mem::take(&mut self.views);
        view_system.render(ctx, self);
        self.views = view_system;
    }

    fn update_windows(&mut self, ctx: &egui::Context) {
        let mut window_system = std::mem::take(&mut self.windows);
        window_system.update(ctx, self);
        self.windows = window_system;
    }

    fn update_file_picker(&mut self, ctx: &egui::Context) {
        let mut file_picker = std::mem::take(&mut self.file_picker);
        file_picker.update(ctx, self);
        self.file_picker = file_picker;
    }

    fn handle_keyboard_inputs(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.key_pressed(egui::Key::F3)) {
            self.windows.debug.is_open = !self.windows.debug.is_open;
        }
    }

    fn handle_app_action(&mut self, ctx: &egui::Context, action: AppAction) {
        let result = match action {
            AppAction::SaveAppSnapshot(path) => self.handle_save_app_snapshot(path, ctx),
            AppAction::RestoreAppSnapshot(path) => self.handle_restore_app_snapshot(path, ctx),
            AppAction::DumpAppJSON(path) => self.handle_dump_app_json(path),
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
}
