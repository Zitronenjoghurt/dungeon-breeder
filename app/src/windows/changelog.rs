use crate::components::changelog::ChangelogComponent;
use crate::components::{Component, EnumSelect};
use crate::data::changelogs::ChangelogVersion;
use crate::windows::ViewWindow;
use egui::{Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ChangelogWindowState {
    pub is_open: bool,
    pub selected_changelog: ChangelogVersion,
}

pub struct ChangelogWindow<'a> {
    state: &'a mut ChangelogWindowState,
}

impl<'a> ChangelogWindow<'a> {
    pub fn new(state: &'a mut ChangelogWindowState) -> Self {
        Self { state }
    }
}

impl ViewWindow for ChangelogWindow<'_> {
    fn id(&self) -> egui::Id {
        egui::Id::new("changelog_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Changelog"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        EnumSelect::new(
            &mut self.state.selected_changelog,
            "changelog_version_select",
        )
        .ui(ui);
        ChangelogComponent::new(self.state.selected_changelog.get_changelog()).ui(ui);
    }
}
