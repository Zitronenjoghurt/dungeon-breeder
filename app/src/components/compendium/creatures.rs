use crate::components::value_button::ValueButton;
use crate::components::{Component, CreatureSprite};
use crate::systems::textures::TextureSystem;
use dungeon_breeder_core::data::creature::id::CreatureID;
use dungeon_breeder_core::Game;
use egui::{Grid, ScrollArea};
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
                ScrollArea::vertical().show(ui, |ui| {
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
                        })
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
