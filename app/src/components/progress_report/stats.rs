use crate::components::Component;
use dungeon_breeder_core::update_report::GameUpdateProgressReport;
use egui::{Grid, ScrollArea};

pub struct ProgressReportStatsComponent<'a> {
    report: &'a GameUpdateProgressReport,
}

impl<'a> ProgressReportStatsComponent<'a> {
    pub fn new(report: &'a GameUpdateProgressReport) -> Self {
        Self { report }
    }
}

impl Component for ProgressReportStatsComponent<'_> {
    fn ui(self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.heading("Statistics");
            ui.group(|ui| {
                ui.set_height(ui.available_height());
                ScrollArea::vertical()
                    .id_salt("progress_report_stats_scroll_area")
                    .show(ui, |ui| {
                        Grid::new("progress_report_stats_grid")
                            .num_columns(2)
                            .striped(true)
                            .min_col_width(ui.available_width() / 2.0)
                            .show(ui, |ui| {
                                ui.label("Specimen slain");
                                ui.label(self.report.times_specimen_slain.to_string());
                                ui.end_row();
                            });
                    });
            });
        });
    }
}
