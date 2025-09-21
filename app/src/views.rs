use crate::app::GameApp;
use egui::Context;
use serde::{Deserialize, Serialize};

mod game;
mod main_menu;

#[derive(Debug, Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum ViewID {
    Game,
    #[default]
    MainMenu,
}

pub trait View {
    fn render(&mut self, ctx: &Context, app: &mut GameApp);
}

#[derive(Default, Serialize, Deserialize)]
pub struct ViewSystem {
    current_view: ViewID,
    pub game: game::GameView,
    pub main_menu: main_menu::MainMenuView,
}

impl ViewSystem {
    pub fn current_view(&self) -> ViewID {
        self.current_view
    }

    pub fn switch_view(&mut self, view: ViewID) {
        self.current_view = view;
    }
}

impl View for ViewSystem {
    fn render(&mut self, ctx: &Context, app: &mut GameApp) {
        match self.current_view {
            ViewID::Game => self.game.render(ctx, app),
            ViewID::MainMenu => self.main_menu.render(ctx, app),
        }
    }
}
