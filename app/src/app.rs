use crate::modals::ModalSystem;
use crate::systems::settings::SettingsSystem;
use crate::systems::textures::TextureSystem;
use crate::views::{View, ViewSystem};
use crate::windows::WindowSystem;
use anyhow::anyhow;
use dungeon_breeder_core::Game;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct GameApp {
    pub game: Game,
    pub settings: SettingsSystem,
    #[serde(skip, default)]
    pub textures: TextureSystem,
    pub modals: ModalSystem,
    pub views: ViewSystem,
    pub windows: WindowSystem,
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
}

impl eframe::App for GameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        self.game.update();
        self.settings.update(ctx);

        let mut view_system = std::mem::take(&mut self.views);
        view_system.render(ctx, self);
        self.views = view_system;

        let mut modal_system = std::mem::take(&mut self.modals);
        modal_system.update(ctx, self);
        self.modals = modal_system;

        let mut window_system = std::mem::take(&mut self.windows);
        window_system.update(ctx, self);
        self.windows = window_system;
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
