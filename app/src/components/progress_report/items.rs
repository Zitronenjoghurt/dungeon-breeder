use crate::components::Component;
use dungeon_breeder_core::update_report::GameUpdateProgressReport;
use egui::{Grid, ScrollArea, Ui};

pub struct ProgressReportItemsComponent<'a> {
    report: &'a GameUpdateProgressReport,
}

impl<'a> ProgressReportItemsComponent<'a> {
    pub fn new(report: &'a GameUpdateProgressReport) -> Self {
        Self { report }
    }
}

impl Component for ProgressReportItemsComponent<'_> {
    fn ui(self, ui: &mut Ui) {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.heading("Items obtained");
            ui.group(|ui| {
                ui.set_height(ui.available_height());
                ScrollArea::vertical()
                    .id_salt("progress_report_items_scroll_area")
                    .show(ui, |ui| {
                        Grid::new("progress_report_items_grid")
                            .num_columns(2)
                            .striped(true)
                            .min_col_width(ui.available_width() / 2.0)
                            .show(ui, |ui| {
                                for (item, amount) in &self.report.items {
                                    ui.label(item.def().name);
                                    ui.label(amount.to_string());
                                    ui.end_row();
                                }
                            });
                    });
            });
        });
    }
}
