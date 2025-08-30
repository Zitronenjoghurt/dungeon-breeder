use crate::state::AppState;
use egui::Context;
use serde::{Deserialize, Serialize};

mod game;

#[derive(Debug, Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum ViewID {
    #[default]
    Game,
}

pub trait View: Default {
    fn render(&mut self, ctx: &Context, state: &mut AppState);
}

#[derive(Default, Serialize, Deserialize)]
pub struct ViewManager {
    game_view: game::GameView,
}

impl View for ViewManager {
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        match state.current_view() {
            ViewID::Game => self.game_view.render(ctx, state),
        }
    }
}
