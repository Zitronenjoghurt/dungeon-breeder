use crate::components::{Component, ToggleButton};
use crate::modals::ModalSystem;
use crate::windows::debug_window::dungeon::DebugDungeonWindow;
use crate::windows::debug_window::items::DebugItemsWindow;
use crate::windows::debug_window::specimen::{DebugSpecimenWindow, DebugSpecimenWindowState};
use crate::windows::ViewWindow;
use dungeon_breeder_core::data::creature::id::CreatureID;
use dungeon_breeder_core::Game;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

mod dungeon;
mod items;
mod specimen;

#[derive(Default, Serialize, Deserialize)]
pub struct DebugWindowState {
    pub is_open: bool,
    pub dungeon_window_open: bool,
    pub items_window_open: bool,
    pub specimen_window: DebugSpecimenWindowState,
}

pub struct DebugWindow<'a> {
    modals: &'a mut ModalSystem,
    game: &'a Game,
    state: &'a mut DebugWindowState,
}

impl<'a> DebugWindow<'a> {
    pub fn new(
        modals: &'a mut ModalSystem,
        game: &'a Game,
        state: &'a mut DebugWindowState,
    ) -> Self {
        Self {
            modals,
            game,
            state,
        }
    }
}

impl ViewWindow for DebugWindow<'_> {
    fn id(&self) -> Id {
        Id::new("debug_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Debug"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        DebugDungeonWindow::new(self.modals, self.game, &mut self.state.dungeon_window_open)
            .show(ui.ctx());
        DebugItemsWindow::new(self.game, &mut self.state.items_window_open).show(ui.ctx());
        DebugSpecimenWindow::new(self.modals, self.game, &mut self.state.specimen_window)
            .show(ui.ctx());

        ui.horizontal(|ui| {
            ToggleButton::new(&mut self.state.dungeon_window_open, "Dungeon").ui(ui);
            ToggleButton::new(&mut self.state.items_window_open, "Items").ui(ui);
            ToggleButton::new(&mut self.state.specimen_window.is_open, "Specimen").ui(ui);
        });

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Random Gonk").clicked() {
                self.game.actions.random_specimen(CreatureID::Gonk);
            }
            if ui.button("Eat the Rich").clicked() {
                self.game.actions.add_coins(1_000_000);
            }
        });
    }
}
