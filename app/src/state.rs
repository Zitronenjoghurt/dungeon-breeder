use crate::state::settings::SettingsState;
use crate::views::ViewID;
use dungeon_breeder_core::Game;
use serde::{Deserialize, Serialize};

pub mod settings;

#[derive(Default, Serialize, Deserialize)]
pub struct AppState {
    game: Game,
    current_view: ViewID,
    settings: SettingsState,
}

impl AppState {
    pub fn update(&mut self, ctx: &egui::Context) {
        self.game.update();
        self.settings.update(ctx);
    }

    pub fn current_view(&self) -> ViewID {
        self.current_view
    }

    pub fn switch_view(&mut self, view: ViewID) {
        self.current_view = view;
    }

    pub fn game(&self) -> &Game {
        &self.game
    }

    pub fn settings_mut(&mut self) -> &mut SettingsState {
        &mut self.settings
    }
}
