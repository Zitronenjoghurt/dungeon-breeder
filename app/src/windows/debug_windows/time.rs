use crate::windows::ViewWindow;
use dungeon_breeder_core::Game;
use egui::{Id, Ui, WidgetText};

pub struct TimeDebugWindow<'a> {
    pub game: &'a Game,
    pub is_open: &'a mut bool,
}

impl<'a> TimeDebugWindow<'a> {
    pub fn new(game: &'a Game, is_open: &'a mut bool) -> Self {
        Self { game, is_open }
    }
}

impl ViewWindow for TimeDebugWindow<'_> {
    fn id(&self) -> Id {
        Id::new("time_debug_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Time Debug"
    }

    fn is_open(&self) -> bool {
        *self.is_open
    }

    fn set_open(&mut self, open: bool) {
        *self.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("1m").clicked() {
                self.game.actions.debug_skip_time(60);
            }
            if ui.button("10m").clicked() {
                self.game.actions.debug_skip_time(600);
            }
            if ui.button("1h").clicked() {
                self.game.actions.debug_skip_time(3600);
            }
            if ui.button("6h").clicked() {
                self.game.actions.debug_skip_time(21600);
            }
            if ui.button("1d").clicked() {
                self.game.actions.debug_skip_time(86400);
            }
            if ui.button("1w").clicked() {
                self.game.actions.debug_skip_time(604800);
            }
            if ui.button("1mo").clicked() {
                self.game.actions.debug_skip_time(2592000);
            }
        });
    }
}
