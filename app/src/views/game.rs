use crate::components::*;
use crate::state::AppState;
use crate::views::View;
use crate::windows::settings::SettingsWindow;
use crate::windows::ViewWindow;
use dungeon_breeder_core::data::creature::id::CreatureID;
use dungeon_breeder_core::state::specimen::SpecimenId;
use egui::{CentralPanel, Context, ScrollArea, TopBottomPanel, Window};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct GameView {
    settings_window_open: bool,
    selected_specimen_id_a: SpecimenId,
    selected_specimen_id_b: SpecimenId,
}

impl View for GameView {
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        SettingsWindow::new(&mut self.settings_window_open, state.settings_mut()).show(ctx);

        self.render_specimen_window(ctx, state);
        self.render_item_window(ctx, state);
        self.render_dungeon_window(ctx, state);

        TopBottomPanel::top("game_tab_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ToggleButton::new(&mut self.settings_window_open, " 🛠 ").ui(ui);
                if ui.button("Random Gonk").clicked() {
                    state.game().actions.random_specimen(CreatureID::Gonk);
                }
            });
        });

        CentralPanel::default().show(ctx, |ui| {});
    }
}

impl GameView {
    fn render_specimen_window(&mut self, ctx: &Context, state: &mut AppState) {
        Window::new("Specimen").show(ctx, |ui| {
            SpecimenSelection::new(
                &state.game().state.specimen,
                &mut self.selected_specimen_id_a,
            )
            .id("select_specimen_a")
            .ui(ui);

            SpecimenSelection::new(
                &state.game().state.specimen,
                &mut self.selected_specimen_id_b,
            )
            .id("select_specimen_b")
            .ui(ui);

            if ui.button("Breed").clicked() {
                state
                    .game()
                    .actions
                    .breed(self.selected_specimen_id_a, self.selected_specimen_id_b);
            }

            if ui.button("Fuse").clicked() {
                state
                    .game()
                    .actions
                    .fuse(self.selected_specimen_id_a, self.selected_specimen_id_b);
            }

            ScrollArea::vertical().show(ui, |ui| {
                SpecimenTable::new(state.game().state.specimen.iter()).ui(ui);
            });
        });
    }

    fn render_item_window(&mut self, ctx: &Context, state: &mut AppState) {
        Window::new("Items").show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ItemTable::new(&state.game().state.items).ui(ui);
            })
        });
    }

    fn render_dungeon_window(&mut self, ctx: &Context, state: &mut AppState) {
        Window::new("Dungeon").show(ctx, |ui| {
            DungeonView::new(state.game(), &state.game().state.dungeon).ui(ui);
        });
    }
}
