use crate::components::dungeon::layer::DungeonLayerView;
use crate::components::Component;
use dungeon_breeder_core::state::dungeon::Dungeon;
use dungeon_breeder_core::Game;
use egui::{Frame, ScrollArea, Ui};

pub struct DungeonView<'a> {
    game: &'a Game,
    dungeon: &'a Dungeon,
    id: &'a str,
}

impl<'a> DungeonView<'a> {
    pub fn new(game: &'a Game, dungeon: &'a Dungeon) -> Self {
        Self {
            game,
            dungeon,
            id: "dungeon_view",
        }
    }

    pub fn id(&self) -> &str {
        self.id
    }
}

impl Component for DungeonView<'_> {
    fn ui(self, ui: &mut Ui) {
        ScrollArea::vertical().id_salt(self.id).show(ui, |ui| {
            for (i, layer) in self.dungeon.iter_layers().enumerate() {
                ui.push_id(i, |ui| {
                    DungeonLayerView::new(self.game, layer, i).ui(ui);
                });
            }
            if let Some(unlock_costs) = self.dungeon.next_layer_costs() {
                Frame::group(ui.style()).show(ui, |ui| {
                    ui.label(format!("{}💰", unlock_costs));
                    if ui.button("Unlock").clicked() {
                        self.game.actions.unlock_dungeon_layer();
                    }
                });
            }
        });
    }
}
