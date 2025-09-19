use crate::utils::formatting::{format_date, format_seconds};
use crate::windows::ViewWindow;
use dungeon_breeder_core::state::statistics::GameStatistics;
use egui::{Grid, Id, ScrollArea, Ui, WidgetText};

pub struct StatisticsWindow<'a> {
    statistics: &'a GameStatistics,
    is_open: &'a mut bool,
}

impl<'a> StatisticsWindow<'a> {
    pub fn new(statistics: &'a GameStatistics, is_open: &'a mut bool) -> Self {
        Self {
            statistics,
            is_open,
        }
    }
}

impl ViewWindow for StatisticsWindow<'_> {
    fn id(&self) -> Id {
        Id::new("statistics_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Statistics"
    }

    fn is_open(&self) -> bool {
        *self.is_open
    }

    fn set_open(&mut self, open: bool) {
        *self.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            Grid::new("statistics_grid")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Started at");
                    ui.label(format_date(self.statistics.started_at));
                    ui.end_row();

                    ui.label("Playtime");
                    ui.label(format_seconds(self.statistics.active_ticks));
                    ui.end_row();

                    ui.label("Idle time");
                    ui.label(format_seconds(self.statistics.passive_ticks));
                    ui.end_row();

                    ui.label("Times bred");
                    ui.label(self.statistics.times_bred.to_string());
                    ui.end_row();

                    ui.label("Times fused");
                    ui.label(self.statistics.times_fused.to_string());
                    ui.end_row();

                    ui.label("Times specimen slain");
                    ui.label(self.statistics.times_specimen_slain.to_string());
                    ui.end_row();
                });
        });
    }
}
