use crate::components::value_button::ValueButton;
use crate::components::Component;
use crate::utils::formatting::{format_date, format_number};
use dungeon_breeder_core::data::item::id::ItemID;
use dungeon_breeder_core::Game;
use egui::{Grid, ScrollArea};
use egui_phosphor::regular;
use strum::IntoEnumIterator;

pub struct CompendiumItemsComponent<'a> {
    game: &'a Game,
    selected_item: &'a mut ItemID,
}

impl<'a> CompendiumItemsComponent<'a> {
    pub fn new(game: &'a Game, selected_item: &'a mut ItemID) -> Self {
        Self {
            game,
            selected_item,
        }
    }

    pub fn show_item_buttons(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.vertical(|ui| {
                for item_id in ItemID::iter() {
                    ui.push_id(format!("compendium_item_button_{item_id}"), |ui| {
                        let is_unlocked = self.game.state.items.compendium().has_unlocked(&item_id);

                        let name = if is_unlocked {
                            item_id.def().name
                        } else {
                            "???"
                        };

                        ValueButton::new(self.selected_item, item_id, name)
                            .enabled(is_unlocked)
                            .ui(ui);
                    });
                }
            });
        });
    }

    pub fn show_item_info(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.group(|ui| {
                ui.set_width(250.0);

                ScrollArea::vertical().show(ui, |ui| {
                    Grid::new("compendium_items_info_grid")
                        .num_columns(2)
                        .striped(true)
                        .min_col_width(121.0)
                        .max_col_width(121.0)
                        .show(ui, |ui| {
                            ui.label("Name");
                            ui.label(self.selected_item.def().name);
                            ui.end_row();

                            ui.label("Price");
                            ui.label(format!(
                                "{} {}",
                                format_number(self.selected_item.def().price),
                                regular::COINS
                            ));
                            ui.end_row();
                        });

                    if let Some(entry) = self
                        .game
                        .state
                        .items
                        .compendium()
                        .get_entry(self.selected_item)
                    {
                        ui.separator();

                        Grid::new("compendium_items_statistics_grid")
                            .num_columns(2)
                            .striped(true)
                            .min_col_width(121.0)
                            .max_col_width(121.0)
                            .show(ui, |ui| {
                                ui.label("Unlocked at");
                                ui.label(format_date(entry.unlocked_at));
                                ui.end_row();

                                ui.label("Times obtained");
                                ui.label(format_number(entry.times_obtained));
                                ui.end_row();

                                ui.label("Times sold");
                                ui.label(format_number(entry.times_sold));
                                ui.end_row();
                            });
                    }
                });
            });
        });
    }
}

impl Component for CompendiumItemsComponent<'_> {
    fn ui(mut self, ui: &mut egui::Ui) {
        ui.horizontal_top(|ui| {
            ui.set_min_height(ui.available_height());

            self.show_item_buttons(ui);

            ui.separator();

            if self
                .game
                .state
                .items
                .compendium()
                .has_unlocked(self.selected_item)
            {
                self.show_item_info(ui);
            }
        });
    }
}
