use crate::components::dungeon::legacy::layer::DungeonLayerView;
use crate::components::Component;
use crate::modals::ModalSystem;
use dungeon_breeder_core::state::dungeon::Dungeon;
use dungeon_breeder_core::Game;
use egui::{Frame, ScrollArea, Ui};
use egui_phosphor::regular;

pub struct LayeredDungeonView<'a> {
    modal_system: &'a mut ModalSystem,
    game: &'a Game,
    dungeon: &'a Dungeon,
    id: &'a str,
}

impl<'a> LayeredDungeonView<'a> {
    pub fn new(modal_system: &'a mut ModalSystem, game: &'a Game, dungeon: &'a Dungeon) -> Self {
        Self {
            modal_system,
            game,
            dungeon,
            id: "dungeon_view",
        }
    }

    pub fn id(&self) -> &str {
        self.id
    }
}

impl Component for LayeredDungeonView<'_> {
    fn ui(self, ui: &mut Ui) {
        ScrollArea::vertical().id_salt(self.id).show(ui, |ui| {
            for (i, layer) in self.dungeon.iter_layers().enumerate() {
                ui.push_id(i, |ui| {
                    DungeonLayerView::new(self.modal_system, self.game, layer, i).ui(ui);
                });
            }
            if let Some(unlock_costs) = self.dungeon.next_layer_costs() {
                Frame::group(ui.style()).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("{} {}", unlock_costs, regular::COINS));
                        if ui.button(regular::LOCK_KEY_OPEN).clicked() {
                            self.game.actions.unlock_dungeon_layer();
                        }
                    });
                });
            }
        });
    }
}
