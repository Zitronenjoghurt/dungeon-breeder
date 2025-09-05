use crate::modals::ModalSystem;
use crate::systems::settings::SettingsSystem;
use crate::systems::textures::TextureSystem;
use crate::systems::toasts::ToastSystem;
use crate::views::{View, ViewSystem};
use crate::windows::WindowSystem;
use anyhow::anyhow;
use dungeon_breeder_core::Game;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct GameApp {
    pub game: Game,
    pub settings: SettingsSystem,
    pub modals: ModalSystem,
    pub views: ViewSystem,
    pub windows: WindowSystem,
    #[serde(skip, default)]
    pub textures: TextureSystem,
    #[serde(skip, default)]
    pub toasts: ToastSystem,
}

impl GameApp {
    pub fn new(cc: &eframe::CreationContext) -> anyhow::Result<Self> {
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

    fn update_game(&mut self) {
        let report = self.game.update();
        if report.state_report.ticks_elapsed > 0 {
            println!("{:?}", report.state_report);
        }

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
}

impl eframe::App for GameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        self.update_game();
        self.update_views(ctx);
        self.update_windows(ctx);
        self.update_modals(ctx);
        self.settings.update(ctx);
        self.toasts.update(ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
