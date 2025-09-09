use crate::app::GameApp;
use crate::windows::ViewWindow;
use egui::{Id, Ui, WidgetText};

pub struct BugReportWindow<'a> {
    app: &'a GameApp,
    is_open: &'a mut bool,
}

impl<'a> BugReportWindow<'a> {
    pub fn new(app: &'a GameApp, is_open: &'a mut bool) -> Self {
        Self { app, is_open }
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
        *self.is_open
    }

    fn set_open(&mut self, open: bool) {
        *self.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {}
}
