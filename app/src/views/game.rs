use crate::components::*;
use crate::state::AppState;
use crate::views::View;
use crate::windows::settings::SettingsWindow;
use dungeon_breeder_core::creature::specimen::SpecimenId;
use egui::{CentralPanel, Context, TopBottomPanel};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct GameView {
    settings_window: SettingsWindow,
    selected_specimen_id_a: SpecimenId,
    selected_specimen_id_b: SpecimenId,
}

impl View for GameView {
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        WindowRenderer::new(ctx, state)
            .window(&mut self.settings_window)
            .render();

        TopBottomPanel::top("game_tab_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                WindowButton::new(&mut self.settings_window, " 🛠 ").ui(ui);
                if ui.button("Random Gonk").clicked() {
                    state.game().actions.random_specimen(0);
                }
            });
        });

        let game_data = &state.game().data;
        let game_state = &state.game().state;
        CentralPanel::default().show(ctx, |ui| {
            SpecimenTable::new(&state.game().data, state.game().state.specimen.iter()).ui(ui);

            SpecimenSelection::new(
                game_data,
                &game_state.specimen,
                &mut self.selected_specimen_id_a,
            )
            .id("select_specimen_a")
            .ui(ui);

            SpecimenSelection::new(
                game_data,
                &game_state.specimen,
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
        });
    }
}
