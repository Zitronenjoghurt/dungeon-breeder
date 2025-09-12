use crate::app::bug_report::BugReportMetadata;
use crate::components::Component;
use egui::{Grid, TextEdit, Ui};

pub struct BugReportMetaEdit<'a> {
    meta: &'a mut BugReportMetadata,
}

impl<'a> BugReportMetaEdit<'a> {
    pub fn new(meta: &'a mut BugReportMetadata) -> Self {
        Self { meta }
    }
}

impl Component for BugReportMetaEdit<'_> {
    fn ui(self, ui: &mut Ui) {
        Grid::new("bug_report_meta_edit_grid")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Title");
                ui.add(TextEdit::singleline(&mut self.meta.title).char_limit(100));
                ui.end_row();

                ui.label("Description");
                ui.add(TextEdit::multiline(&mut self.meta.description).char_limit(5000));
                ui.end_row();
            });
    }
}
