use crate::app::GameApp;
use crate::types::color::ColorConvert;
use crate::utils::formatting::format_seconds;
use crate::windows::ViewWindow;
use dungeon_breeder_core::data::config::CONFIG;
use dungeon_breeder_core::data::creature::id::CreatureID;
use egui::{Button, Id, ProgressBar, ScrollArea, Ui, WidgetText};
use egui_phosphor::regular;

pub struct SummonWindow<'a> {
    app: &'a GameApp,
    is_open: &'a mut bool,
}

impl<'a> SummonWindow<'a> {
    pub fn new(app: &'a GameApp, is_open: &'a mut bool) -> Self {
        Self { app, is_open }
    }

    fn show_creature(&self, ui: &mut Ui, creature_id: CreatureID) {
        let Some(cooldown) = creature_id.def().summoning_cooldown else {
            return;
        };

        let Some(progress) = self
            .app
            .game
            .state
            .summoning
            .get_cooldown_progress(creature_id, &self.app.game.state.specimen)
        else {
            return;
        };

        let secs_left = ((1.0 - progress) * cooldown as f32) as u64;
        ui.group(|ui| {
            ui.set_width(200.0);

            ui.horizontal(|ui| {
                let button_response = ui.add_enabled(progress >= 1.0, Button::new(regular::FLAME));
                if button_response.clicked() {
                    self.app.game.actions.summon_creature(creature_id);
                }

                let text = if progress < 1.0 {
                    format_seconds(secs_left)
                } else {
                    "Ready".to_string()
                };

                ui.add(
                    ProgressBar::new(progress)
                        .corner_radius(1.0)
                        .text(text)
                        .fill(CONFIG.styles.color_intelligence.to_egui())
                        .desired_width(100.0),
                );

                ui.label(creature_id.def().name);
            });
        });
    }
}

impl ViewWindow for SummonWindow<'_> {
    fn id(&self) -> Id {
        Id::new("summon_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Summoning"
    }

    fn is_open(&self) -> bool {
        *self.is_open
    }

    fn set_open(&mut self, open: bool) {
        *self.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            for creature_id in CreatureID::iter_summonable() {
                self.show_creature(ui, creature_id);
            }
        });
    }
}
