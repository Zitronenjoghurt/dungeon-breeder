use crate::components::Component;
use dungeon_breeder_core::state::item::collection::ItemCollection;
use eframe::emath::Align;
use egui::{Layout, Ui};
use egui_extras::{Column, TableBuilder};

pub struct ItemTable<'a> {
    item_collection: &'a ItemCollection,
}

impl<'a> ItemTable<'a> {
    pub fn new(item_collection: &'a ItemCollection) -> Self {
        Self { item_collection }
    }
}

impl Component for ItemTable<'_> {
    fn ui(self, ui: &mut Ui) {
        let text_height = ui.text_style_height(&egui::TextStyle::Body);

        TableBuilder::new(ui)
            .striped(true)
            .cell_layout(Layout::left_to_right(Align::Center))
            .column(Column::auto().at_least(50.0))
            .column(Column::auto().at_least(30.0))
            .header(text_height, |mut header| {
                header.col(|ui| {
                    ui.label("Name");
                });

                header.col(|ui| {
                    ui.label("Count");
                });
            })
            .body(|mut body| {
                for (item_id, count) in self.item_collection.iter() {
                    body.row(text_height, |mut row| {
                        row.col(|ui| {
                            ui.label(item_id.def().name);
                        });

                        row.col(|ui| {
                            ui.label(format!("{}", count));
                        });
                    })
                }
            });
    }
}
