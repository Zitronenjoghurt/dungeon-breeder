use crate::app::bug_report::BugReportMetadata;
use crate::app::system_info::SystemInfo;
use crate::app::GameApp;
use crate::components::system_info::SystemInfoComponent;
use crate::components::{BugReportMetaEdit, Component};
use crate::systems::file_picker::FilePickerConfig;
use crate::windows::ViewWindow;
use egui::{CollapsingHeader, Context, Id, Ui, WidgetText};
use egui_phosphor::regular;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BugReportWindowState {
    pub is_open: bool,
    pub metadata: BugReportMetadata,
    pub system_info_open: bool,
    #[serde(skip, default)]
    pub system_info: Option<SystemInfo>,
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

    fn before_close(&mut self, _ctx: &Context) {
        self.state.system_info = None;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        self.state.system_info = Some(SystemInfo::collect());

        BugReportMetaEdit::new(&mut self.state.metadata).ui(ui);

        if let Some(system_info) = &self.state.system_info {
            ui.group(|ui| {
                ui.set_width(ui.available_width());
                CollapsingHeader::new("System information")
                    .id_salt("bug_report_system_info_collapsible")
                    .show(ui, |ui| {
                        SystemInfoComponent::new(system_info).ui(ui);
                    });
                ui.small(format!(
                    "{} This information will be included in the bug report",
                    regular::INFO
                ))
            });
        }

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
