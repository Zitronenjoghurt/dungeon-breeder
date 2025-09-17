use crate::app::GameApp;
use crate::components::system_info::SystemInfoComponent;
use crate::components::{BugReportMetaEdit, Component};
use crate::systems::file_picker::FilePickerConfig;
use crate::windows::ViewWindow;
use egui::{CollapsingHeader, Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BugReportDebugWindowState {
    pub is_open: bool,
}

pub struct BugReportDebugWindow<'a> {
    app: &'a mut GameApp,
    state: &'a mut BugReportDebugWindowState,
}

impl<'a> BugReportDebugWindow<'a> {
    pub fn new(app: &'a mut GameApp, state: &'a mut BugReportDebugWindowState) -> Self {
        Self { app, state }
    }
}

impl ViewWindow for BugReportDebugWindow<'_> {
    fn id(&self) -> Id {
        Id::new("bug_report_debug_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Bug Report Debug"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("Review").clicked() {
                self.app
                    .file_picker
                    .open(FilePickerConfig::pick_single(), |app, paths| {
                        let Some(path) = paths.first() else {
                            return;
                        };
                        app.actions.review_bug_report(path);
                    });
            }

            if ui.button("Reset").clicked() {
                self.app.bug_report_review.reset();
            }

            if self.app.bug_report_review.bug_report().is_some() && ui.button("Restore").clicked() {
                self.app.actions.restore_bug_report();
            }
        });

        if let Some(bug_report) = self.app.bug_report_review.bug_report_mut() {
            ui.group(|ui| {
                ui.set_width(ui.available_width());
                CollapsingHeader::new("Metadata")
                    .id_salt("bug_report_review_metadata_collapsible")
                    .show(ui, |ui| {
                        BugReportMetaEdit::new(&mut bug_report.metadata)
                            .allow_edit(false)
                            .ui(ui);
                    });
            });

            ui.group(|ui| {
                ui.set_width(ui.available_width());
                CollapsingHeader::new("System information")
                    .id_salt("bug_report_review_system_information_collapsible")
                    .show(ui, |ui| {
                        SystemInfoComponent::new(&bug_report.system).ui(ui);
                    });
            });
        }
    }
}
