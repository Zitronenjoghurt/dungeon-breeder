use crate::app::bug_report::BugReportMetadata;
use crate::app::GameApp;
use crate::components::{BugReportMetaEdit, Component};
use crate::systems::file_picker::FilePickerConfig;
use crate::windows::ViewWindow;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BugReportWindowState {
    pub is_open: bool,
    pub metadata: BugReportMetadata,
}

pub struct BugReportWindow<'a> {
    app: &'a mut GameApp,
    state: &'a mut BugReportWindowState,
}

impl<'a> BugReportWindow<'a> {
    pub fn new(app: &'a mut GameApp, state: &'a mut BugReportWindowState) -> Self {
        Self { app, state }
    }
}

impl ViewWindow for BugReportWindow<'_> {
    fn id(&self) -> Id {
        Id::new("bug_report_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Bug Report"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        BugReportMetaEdit::new(&mut self.state.metadata).ui(ui);
        ui.vertical_centered(|ui| {
            if ui.button("Create").clicked() {
                let metadata = self.state.metadata.clone();
                self.app.file_picker.open(
                    FilePickerConfig::save().default_file_name("bug_report.dbrep"),
                    move |app, paths| {
                        let Some(path) = paths.first() else { return };
                        app.actions.create_bug_report(path, metadata);
                    },
                );
            }
        });
    }
}
