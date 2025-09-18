use crate::components::compendium::creatures::CompendiumCreaturesComponent;
use crate::components::compendium::items::CompendiumItemsComponent;
use crate::components::value_button::ValueButton;
use crate::components::Component;
use crate::systems::textures::TextureSystem;
use dungeon_breeder_core::data::creature::id::CreatureID;
use dungeon_breeder_core::data::item::id::ItemID;
use dungeon_breeder_core::Game;
use egui::Ui;
use serde::{Deserialize, Serialize};

mod creatures;
mod items;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompendiumTab {
    #[default]
    Creatures,
    Items,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CompendiumComponentState {
    tab: CompendiumTab,
    selected_creature: CreatureID,
    selected_item: ItemID,
}

pub struct CompendiumComponent<'a> {
    textures: &'a mut TextureSystem,
    game: &'a Game,
    state: &'a mut CompendiumComponentState,
}

impl<'a> CompendiumComponent<'a> {
    pub fn new(
        textures: &'a mut TextureSystem,
        game: &'a Game,
        state: &'a mut CompendiumComponentState,
    ) -> Self {
        Self {
            textures,
            game,
            state,
        }
    }

    pub fn show_tab_buttons(&mut self, ui: &mut egui::Ui) {
        ValueButton::new(&mut self.state.tab, CompendiumTab::Creatures, "Creatures").ui(ui);
        ValueButton::new(&mut self.state.tab, CompendiumTab::Items, "Items").ui(ui);
    }
}

impl Component for CompendiumComponent<'_> {
    fn ui(mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            self.show_tab_buttons(ui);
        });

        ui.separator();

        ui.group(|ui| match self.state.tab {
            CompendiumTab::Creatures => CompendiumCreaturesComponent::new(
                self.textures,
                self.game,
                &mut self.state.selected_creature,
            )
            .ui(ui),
            CompendiumTab::Items => {
                CompendiumItemsComponent::new(self.game, &mut self.state.selected_item).ui(ui)
            }
        });
    }
}
