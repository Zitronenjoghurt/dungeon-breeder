use crate::app::GameApp;
use egui::Context;
use serde::{Deserialize, Serialize};

mod game;

#[derive(Debug, Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum ViewID {
    #[default]
    Game,
}

pub trait View {
    fn render(&mut self, ctx: &Context, app: &mut GameApp);
}

#[derive(Default, Serialize, Deserialize)]
pub struct ViewSystem {
    current_view: ViewID,
    pub game: game::GameView,
}

impl View for ViewSystem {
    fn render(&mut self, ctx: &Context, app: &mut GameApp) {
        match self.current_view {
            ViewID::Game => self.game.render(ctx, app),
        }
    }
}
