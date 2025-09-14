use crate::app::bug_report::BugReportMetadata;
use crate::components::Component;
use egui::{TextEdit, Ui};

pub struct BugReportMetaEdit<'a> {
    meta: &'a mut BugReportMetadata,
    allow_edit: bool,
}

impl<'a> BugReportMetaEdit<'a> {
    pub fn new(meta: &'a mut BugReportMetadata) -> Self {
        Self {
            meta,
            allow_edit: true,
        }
    }

    pub fn allow_edit(mut self, allow_edit: bool) -> Self {
        self.allow_edit = allow_edit;
        self
    }
}

impl Component for BugReportMetaEdit<'_> {
    fn ui(self, ui: &mut Ui) {
        ui.add_enabled(
            self.allow_edit,
            TextEdit::singleline(&mut self.meta.title)
                .char_limit(100)
                .desired_width(ui.available_width())
                .hint_text("Title"),
        );

        ui.add_enabled(
            self.allow_edit,
            TextEdit::multiline(&mut self.meta.description)
                .char_limit(5000)
                .desired_width(ui.available_width())
                .hint_text("Description"),
        );
    }
}
