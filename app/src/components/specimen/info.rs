use crate::components::{Component, CreatureSprite};
use crate::systems::textures::TextureSystem;
use crate::utils::formatting::{format_date, format_seconds};
use dungeon_breeder_core::state::specimen::Specimen;
use egui::{Grid, ScrollArea, Ui};

pub struct SpecimenInfo<'a> {
    textures: &'a mut TextureSystem,
    specimen: Option<&'a Specimen>,
}

impl<'a> SpecimenInfo<'a> {
    pub fn new(textures: &'a mut TextureSystem, specimen: Option<&'a Specimen>) -> Self {
        Self { textures, specimen }
    }
}

impl Component for SpecimenInfo<'_> {
    fn ui(self, ui: &mut Ui) {
        ui.vertical(|ui| {
            if let Some(specimen) = self.specimen {
                CreatureSprite::new(self.textures, Some(specimen.creature_id))
                    .size(300.0)
                    .ui(ui);

                ui.group(|ui| {
                    ui.set_width(300.0);
                    ScrollArea::vertical()
                        .min_scrolled_height(200.0)
                        .id_salt("specimen_info_scroll_area")
                        .show(ui, |ui| {
                            Grid::new("specimen_info_grid")
                                .num_columns(2)
                                .striped(true)
                                .min_col_width(145.0)
                                .max_col_width(145.0)
                                .show(ui, |ui| {
                                    ui.label("ID");
                                    ui.label(specimen.id.to_string());
                                    ui.end_row();

                                    ui.label("Creature");
                                    ui.label(specimen.creature_id.def().name);
                                    ui.end_row();

                                    ui.label("Nickname");
                                    if let Some(nickname) = &specimen.nickname {
                                        ui.label(nickname);
                                    } else {
                                        ui.label("");
                                    }
                                    ui.end_row();

                                    ui.label("Current health");
                                    ui.label(format!("{:0.2}%", specimen.current_health() * 100.0));
                                    ui.end_row();

                                    ui.label("Is regenerating?");
                                    ui.label(format!("{}", specimen.is_regenerating));
                                    ui.end_row();

                                    ui.label("Possible drops");
                                    ui.vertical(|ui| {
                                        if specimen.iter_possible_drops().count() > 0 {
                                            ScrollArea::vertical()
                                                .id_salt(
                                                    "specimen_info_grid_item_drops_scroll_area",
                                                )
                                                .show(ui, |ui| {
                                                    Grid::new("specimen_info_grid_item_drops_grid")
                                                        .num_columns(3)
                                                        .striped(true)
                                                        .max_col_width(50.0)
                                                        .show(ui, |ui| {
                                                            ui.label("Item");
                                                            ui.label("x");
                                                            ui.label("%");
                                                            ui.end_row();

                                                            for drop in
                                                                specimen.iter_possible_drops()
                                                            {
                                                                ui.label(drop.item_id.def().name);
                                                                ui.label(format!(
                                                                    "{:?}",
                                                                    drop.count_range
                                                                ));
                                                                ui.label(format!(
                                                                    "{:0.0}",
                                                                    drop.drop_chance * 100.0
                                                                ));
                                                                ui.end_row();
                                                            }
                                                        });
                                                });
                                        } else {
                                            ui.label("None");
                                        }
                                    });
                                    ui.end_row();

                                    ui.label("Obtain method");
                                    ui.label(specimen.obtain_method.to_string());
                                    ui.end_row();

                                    ui.label("Obtained at");
                                    ui.label(format_date(specimen.obtained_at));
                                    ui.end_row();

                                    ui.label("Power");
                                    ui.label(format!("{:0.2}", specimen.power()));
                                    ui.end_row();

                                    ui.label("Proficiency");
                                    ui.label(format!("{:0.2}%", specimen.proficiency() * 100.0));
                                    ui.end_row();

                                    ui.label("Slay duration");
                                    ui.label(format_seconds(specimen.slay_duration_secs()));
                                    ui.end_row();

                                    ui.label("Regeneration duration");
                                    ui.label(format_seconds(specimen.regeneration_duration_secs()));
                                    ui.end_row();

                                    ui.label("Breeding cooldown");
                                    ui.label(format_seconds(specimen.breeding_duration_secs()));
                                    ui.end_row();

                                    ui.label("Strength");
                                    ui.label(format!("{:0.2}%", specimen.strength * 100.0));
                                    ui.end_row();

                                    ui.label("Intelligence");
                                    ui.label(format!("{:0.2}%", specimen.intelligence * 100.0));
                                    ui.end_row();

                                    ui.label("Vitality");
                                    ui.label(format!("{:0.2}%", specimen.vitality * 100.0));
                                    ui.end_row();

                                    ui.label("Agility");
                                    ui.label(format!("{:0.2}%", specimen.agility * 100.0));
                                    ui.end_row();

                                    ui.label("Regeneration");
                                    ui.label(format!("{:0.2}%", specimen.regeneration * 100.0));
                                    ui.end_row();

                                    ui.label("Fertility");
                                    ui.label(format!("{:0.2}%", specimen.fertility * 100.0));
                                    ui.end_row();

                                    ui.label("Times slain");
                                    ui.label(format!("{}", specimen.times_slain));
                                    ui.end_row();

                                    if let Some(last_bred) = specimen.last_bred {
                                        ui.label("Times bred");
                                        ui.label(format!("{}", specimen.times_bred));
                                        ui.end_row();

                                        ui.label("Last bred at");
                                        ui.label(format_date(last_bred));
                                        ui.end_row();
                                    }

                                    ui.label("Breeding generation");
                                    ui.label(format!("{}", specimen.breeding_generation));
                                    ui.end_row();

                                    ui.label("Fusion generation");
                                    ui.label(format!("{}", specimen.fusion_generation));
                                    ui.end_row();
                                });
                        });
                });
            } else {
                CreatureSprite::new(self.textures, None).ui(ui);
            }
        });
    }
}
