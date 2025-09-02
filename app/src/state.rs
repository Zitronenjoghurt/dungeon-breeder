use crate::modals::ModalSystem;
use crate::state::settings::SettingsState;
use crate::views::ViewID;
use dungeon_breeder_core::Game;
use serde::{Deserialize, Serialize};

pub mod settings;

#[derive(Default, Serialize, Deserialize)]
pub struct AppState {
    pub game: Game,
    current_view: ViewID,
    settings: SettingsState,
    pub modals: ModalSystem,
}

impl AppState {
    pub fn update(&mut self, ctx: &egui::Context) {
        self.game.update();
        self.settings.update(ctx);

        let mut modal_system = std::mem::take(&mut self.modals);
        modal_system.update(ctx, self);
        self.modals = modal_system;
    }

    pub fn current_view(&self) -> ViewID {
        self.current_view
    }

    pub fn switch_view(&mut self, view: ViewID) {
        self.current_view = view;
    }

    pub fn settings_mut(&mut self) -> &mut SettingsState {
        &mut self.settings
    }
}
