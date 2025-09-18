use crate::components::{Component, EnumSelect};
use dungeon_breeder_core::state::specimen::NewSpecimen;
use dungeon_breeder_core::Game;
use egui::{Grid, Slider, Widget};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SpecimenSpawnComponentState {
    new_specimen: NewSpecimen,
}

pub struct SpecimenSpawnComponent<'a> {
    state: &'a mut SpecimenSpawnComponentState,
    game: &'a Game,
}

impl<'a> SpecimenSpawnComponent<'a> {
    pub fn new(state: &'a mut SpecimenSpawnComponentState, game: &'a Game) -> Self {
        Self { state, game }
    }
}

impl Component for SpecimenSpawnComponent<'_> {
    fn ui(self, ui: &mut egui::Ui) {
        Grid::new("specimen_spawn_grid")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Creature");
                EnumSelect::new(
                    &mut self.state.new_specimen.creature_id,
                    "specimen_spawn_enum_select",
                )
                .ui(ui);
                ui.end_row();

                ui.label("Strength");
                Slider::new(&mut self.state.new_specimen.strength, 0.0..=1.0).ui(ui);
                ui.end_row();

                ui.label("Intelligence");
                Slider::new(&mut self.state.new_specimen.intelligence, 0.0..=1.0).ui(ui);
                ui.end_row();

                ui.label("Vitality");
                Slider::new(&mut self.state.new_specimen.vitality, 0.0..=1.0).ui(ui);
                ui.end_row();

                ui.label("Agility");
                Slider::new(&mut self.state.new_specimen.agility, 0.0..=1.0).ui(ui);
                ui.end_row();

                ui.label("Regeneration");
                Slider::new(&mut self.state.new_specimen.regeneration, 0.0..=1.0).ui(ui);
                ui.end_row();

                ui.label("Fertility");
                Slider::new(&mut self.state.new_specimen.fertility, 0.0..=1.0).ui(ui);
                ui.end_row();
            });

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Spawn").clicked() {
                self.game.actions.spawn_specimen(NewSpecimen {
                    breeding_generation: 1,
                    fusion_generation: 1,
                    ..self.state.new_specimen
                })
            }

            if ui.button("Min").clicked() {
                self.state.new_specimen.strength = 0.0;
                self.state.new_specimen.intelligence = 0.0;
                self.state.new_specimen.vitality = 0.0;
                self.state.new_specimen.agility = 0.0;
                self.state.new_specimen.regeneration = 0.0;
                self.state.new_specimen.fertility = 0.0;
            }

            if ui.button("Max").clicked() {
                self.state.new_specimen.strength = 1.0;
                self.state.new_specimen.intelligence = 1.0;
                self.state.new_specimen.vitality = 1.0;
                self.state.new_specimen.agility = 1.0;
                self.state.new_specimen.regeneration = 1.0;
                self.state.new_specimen.fertility = 1.0;
            }
        });
    }
}
