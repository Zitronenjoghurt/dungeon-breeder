use crate::app::GameApp;
use crate::components::progress_report::ProgressReportComponent;
use crate::components::Component;
use crate::modals::AppModal;
use crate::utils::formatting::format_seconds;
use dungeon_breeder_core::update::report::GameUpdateProgressReport;
use eframe::emath::Align;
use egui::{Id, Layout, Ui};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct OfflineProgressModal {
    #[serde(skip, default)]
    progress: Option<GameUpdateProgressReport>,
    #[serde(skip, default)]
    ticks_elapsed: Option<u64>,
}

impl OfflineProgressModal {
    pub fn open(&mut self, progress: GameUpdateProgressReport, ticks_elapsed: u64) {
        self.progress = Some(progress);
        self.ticks_elapsed = Some(ticks_elapsed);
    }
}

impl AppModal for OfflineProgressModal {
    fn id(&self) -> Id {
        Id::new("offline_progress_modal")
    }

    fn is_open(&self) -> bool {
        self.progress.is_some() && self.ticks_elapsed.is_some()
    }

    fn close(&mut self) {
        self.progress = None;
        self.ticks_elapsed = None;
    }

    fn update_content(&mut self, ui: &mut Ui, _app: &mut GameApp) {
        let Some(progress) = &self.progress else {
            return;
        };
        let Some(ticks_elapsed) = &self.ticks_elapsed else {
            return;
        };

        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.heading("Welcome back!");
            ui.label(format!(
                "You were away for: {}",
                format_seconds(*ticks_elapsed)
            ));
            ui.separator();

            ProgressReportComponent::new(progress).ui(ui);
        });
    }
}
