use crate::data::changelogs::Changelog;

pub static CHANGELOG_V0_2_0_ALPHA: Changelog = Changelog {
    title: "First Alpha Feature Update",
    year: 2025,
    month: 10,
    day: 31,
    description: "",
    added: &["Localization"],
    changed: &[],
    removed: &[],
    fixed: &["Bug report window not showing up when opened in the main menu"],
};
