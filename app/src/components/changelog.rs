use crate::components::Component;
use crate::data::changelogs::Changelog;
use egui::{Grid, Label, ScrollArea, Ui};
use egui_phosphor::regular;

pub struct ChangelogComponent<'a> {
    pub changelog: &'a Changelog,
}

impl<'a> ChangelogComponent<'a> {
    pub fn new(changelog: &'a Changelog) -> Self {
        Self { changelog }
    }

    fn show_list(&self, ui: &mut Ui, title: &str, items: &[&str]) {
        ui.heading(title);

        if items.is_empty() {
            ui.label("Nothing");
            return;
        }

        Grid::new(format!("changelog_{}_list", title))
            .striped(true)
            .num_columns(1)
            .show(ui, |ui| {
                for item in items {
                    ui.add(Label::new(format!("{} {}", regular::DOT_OUTLINE, *item)).wrap());
                    ui.end_row();
                }
            });
    }
}

impl Component for ChangelogComponent<'_> {
    fn ui(self, ui: &mut Ui) {
        ui.heading(self.changelog.title);
        ui.label(self.changelog.description);
        ui.separator();
        ScrollArea::vertical().show(ui, |ui| {
            self.show_list(ui, "Added", self.changelog.added);
            ui.separator();
            self.show_list(ui, "Changed", self.changelog.changed);
            ui.separator();
            self.show_list(ui, "Removed", self.changelog.removed);
            ui.separator();
            self.show_list(ui, "Fixed", self.changelog.fixed);
        });
    }
}
