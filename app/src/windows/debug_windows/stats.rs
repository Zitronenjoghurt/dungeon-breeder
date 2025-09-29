use crate::app::GameApp;
use crate::windows::ViewWindow;
use egui::{Grid, Id, Ui, WidgetText};

pub struct DebugStatsWindow<'a> {
    app: &'a GameApp,
    is_open: &'a mut bool,
}

impl<'a> DebugStatsWindow<'a> {
    pub fn new(app: &'a GameApp, is_open: &'a mut bool) -> Self {
        Self { app, is_open }
    }
}

impl ViewWindow for DebugStatsWindow<'_> {
    fn id(&self) -> Id {
        Id::new("debug_stats_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Debug Stats"
    }

    fn is_open(&self) -> bool {
        *self.is_open
    }

    fn set_open(&mut self, open: bool) {
        *self.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        Grid::new("debug_stats_grid").num_columns(4).show(ui, |ui| {
            ui.label("Stat");
            ui.label("Min");
            ui.label("Avg");
            ui.label("Max");
            ui.end_row();

            ui.label("App Update");
            ui.label(format!("{:?}", self.app.debug_stats.app_update_min));
            ui.label(format!("{:?}", self.app.debug_stats.app_update_avg));
            ui.label(format!("{:?}", self.app.debug_stats.app_update_max));
            ui.end_row();

            ui.label("Game Update");
            ui.label(format!("{:?}", self.app.debug_stats.game_update_min));
            ui.label(format!("{:?}", self.app.debug_stats.game_update_avg));
            ui.label(format!("{:?}", self.app.debug_stats.game_update_max));
            ui.end_row();

            ui.label("Game Tick");
            ui.label(format!("{:?}", self.app.debug_stats.game_tick_min));
            ui.label(format!("{:?}", self.app.debug_stats.game_tick_avg));
            ui.label(format!("{:?}", self.app.debug_stats.game_tick_max));
            ui.end_row();
        });
    }
}
