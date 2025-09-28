use crate::data::changelogs::Changelog;

pub static CHANGELOG_V0_1_0_ALPHA: Changelog = Changelog {
    title: "Alpha Release",
    year: 2025,
    month: 9,
    day: 30,
    description: "The very first release, exciting!",
    added: &[
        "Main Menu & Title Screen",
        "Basic game mechanics & architecture",
        "Specimen overview with a sortable and configurable table",
        "Five specimen and their loot items",
        "Basic breeding & fusion system",
        "Basic dungeon system",
        "Tips & Hints system",
        "Aegus & the tutorial dialogue",
    ],
    changed: &[],
    removed: &[],
    fixed: &[],
};
