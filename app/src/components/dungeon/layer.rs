use crate::components::Component;
use crate::modals::ModalSystem;
use dungeon_breeder_core::state::dungeon::layer::DungeonLayer;
use dungeon_breeder_core::Game;

pub struct DungeonLayerComponent<'a> {
    modals: &'a mut ModalSystem,
    game: &'a Game,
    layer: &'a DungeonLayer,
}

impl<'a> DungeonLayerComponent<'a> {
    pub fn new(modals: &'a mut ModalSystem, game: &'a Game, layer: &'a DungeonLayer) -> Self {
        Self {
            modals,
            game,
            layer,
        }
    }
}

impl Component for DungeonLayerComponent<'_> {
    fn ui(self, ui: &mut egui::Ui) {}
}
