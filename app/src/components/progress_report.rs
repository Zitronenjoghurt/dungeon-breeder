use crate::components::Component;
use dungeon_breeder_core::update::report::GameUpdateProgressReport;
use egui::Ui;

mod items;
mod stats;

pub struct ProgressReportComponent<'a> {
    report: &'a GameUpdateProgressReport,
}

impl<'a> ProgressReportComponent<'a> {
    pub fn new(report: &'a GameUpdateProgressReport) -> Self {
        Self { report }
    }
}

impl Component for ProgressReportComponent<'_> {
    fn ui(self, ui: &mut Ui) {
        ui.set_width(500.0);
        ui.columns(2, |columns| {
            columns[0].group(|ui| {
                ui.set_width(250.0);
                ui.set_height(200.0);
                items::ProgressReportItemsComponent::new(self.report).ui(ui);
            });
            columns[1].group(|ui| {
                ui.set_width(250.0);
                ui.set_height(200.0);
                stats::ProgressReportStatsComponent::new(self.report).ui(ui);
            });
        });
    }
}
