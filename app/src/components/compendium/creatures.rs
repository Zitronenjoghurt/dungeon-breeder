use crate::components::value_button::ValueButton;
use crate::components::{Component, CreatureSprite};
use crate::systems::textures::TextureSystem;
use crate::utils::formatting::{format_date, format_number};
use dungeon_breeder_core::data::creature::id::CreatureID;
use dungeon_breeder_core::Game;
use egui::{Grid, Label, ScrollArea};
use strum::IntoEnumIterator;

pub struct CompendiumCreaturesComponent<'a> {
    textures: &'a mut TextureSystem,
    game: &'a Game,
    selected_creature: &'a mut CreatureID,
}

impl<'a> CompendiumCreaturesComponent<'a> {
    pub fn new(
        textures: &'a mut TextureSystem,
        game: &'a Game,
        selected_creature: &'a mut CreatureID,
    ) -> Self {
        Self {
            textures,
            game,
            selected_creature,
        }
    }

    pub fn show_creature_buttons(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.vertical(|ui| {
                for creature_id in CreatureID::iter() {
                    ui.push_id(format!("compendium_creature_button_{creature_id}"), |ui| {
                        let is_unlocked = self
                            .game
                            .state
                            .specimen
                            .compendium()
                            .has_unlocked(&creature_id);

                        let name = if is_unlocked {
                            creature_id.def().name
                        } else {
                            "???"
                        };

                        ValueButton::new(self.selected_creature, creature_id, name)
                            .enabled(is_unlocked)
                            .ui(ui);
                    });
                }
            });
        });
    }

    pub fn show_creature_info(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            CreatureSprite::new(self.textures, Some(*self.selected_creature))
                .size(250.0)
                .ui(ui);
            ui.group(|ui| {
                ui.set_width(250.0);

                ScrollArea::vertical().show(ui, |ui| {
                    ui.add(Label::new(self.selected_creature.def().flavor_text).wrap());
                    ui.separator();

                    Grid::new("compendium_creature_info_grid")
                        .num_columns(2)
                        .striped(true)
                        .min_col_width(121.0)
                        .max_col_width(121.0)
                        .show(ui, |ui| {
                            ui.label("Name");
                            ui.label(self.selected_creature.def().name);
                            ui.end_row();

                            ui.label("Max Power");
                            ui.label(self.selected_creature.def().max_power.to_string());
                            ui.end_row();

                            ui.label("Tier");
                            ui.label(self.selected_creature.def().tier.to_string());
                            ui.end_row();
                        });

                    if let Some(entry) = self
                        .game
                        .state
                        .specimen
                        .compendium()
                        .get_entry(self.selected_creature)
                    {
                        ui.separator();

                        Grid::new("compendium_creature_statistics_grid")
                            .num_columns(2)
                            .striped(true)
                            .min_col_width(121.0)
                            .max_col_width(121.0)
                            .show(ui, |ui| {
                                ui.label("Unlocked at");
                                ui.label(format_date(entry.unlocked_at));
                                ui.end_row();

                                ui.label("Times slain");
                                ui.label(format_number(entry.times_slain));
                                ui.end_row();

                                ui.label("Times bred");
                                ui.label(format_number(entry.times_bred));
                                ui.end_row();

                                ui.label("Times fused");
                                ui.label(format_number(entry.times_fused));
                                ui.end_row();
                            });

                        ui.separator();

                        Grid::new("compendium_creature_specimen_stats_grid")
                            .num_columns(3)
                            .striped(true)
                            .min_col_width(80.0)
                            .max_col_width(80.0)
                            .show(ui, |ui| {
                                ui.label("Stat");
                                ui.label("Min");
                                ui.label("Max");
                                ui.end_row();

                                ui.label("PROF");
                                ui.label(format!("{:.2}%", entry.proficiency.0 * 100.0));
                                ui.label(format!("{:.2}%", entry.proficiency.1 * 100.0));
                                ui.end_row();

                                ui.label("STR");
                                ui.label(format!("{:.2}%", entry.strength.0 * 100.0));
                                ui.label(format!("{:.2}%", entry.strength.1 * 100.0));
                                ui.end_row();

                                ui.label("INT");
                                ui.label(format!("{:.2}%", entry.intelligence.0 * 100.0));
                                ui.label(format!("{:.2}%", entry.intelligence.1 * 100.0));
                                ui.end_row();

                                ui.label("VIT");
                                ui.label(format!("{:.2}%", entry.vitality.0 * 100.0));
                                ui.label(format!("{:.2}%", entry.vitality.1 * 100.0));
                                ui.end_row();

                                ui.label("AGI");
                                ui.label(format!("{:.2}%", entry.agility.0 * 100.0));
                                ui.label(format!("{:.2}%", entry.agility.1 * 100.0));
                                ui.end_row();

                                ui.label("REG");
                                ui.label(format!("{:.2}%", entry.regeneration.0 * 100.0));
                                ui.label(format!("{:.2}%", entry.regeneration.1 * 100.0));
                                ui.end_row();

                                ui.label("FERT");
                                ui.label(format!("{:.2}%", entry.fertility.0 * 100.0));
                                ui.label(format!("{:.2}%", entry.fertility.1 * 100.0));
                                ui.end_row();
                            });
                    }
                });
            });
        });
    }
}

impl Component for CompendiumCreaturesComponent<'_> {
    fn ui(mut self, ui: &mut egui::Ui) {
        ui.horizontal_top(|ui| {
            ui.set_min_height(ui.available_height());

            self.show_creature_buttons(ui);

            ui.separator();

            if self
                .game
                .state
                .specimen
                .compendium()
                .has_unlocked(self.selected_creature)
            {
                self.show_creature_info(ui);
            }
        });
    }
}
