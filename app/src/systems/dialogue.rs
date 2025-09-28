use crate::app::GameApp;
use crate::components::dialogue::dialogue_box::DialogueBoxComponent;
use crate::components::Component;
use dungeon_breeder_core::data::dialogue::options::DialoguePosition;
use eframe::emath::{Align2, Vec2};
use eframe::epaint::Color32;
use egui::{Area, Id, Order, Sense};

pub struct DialogueSystem;

impl DialogueSystem {
    pub fn update(ctx: &egui::Context, app: &mut GameApp) {
        let Ok(entry) = app.game.state.active_dialogue.get_entry() else {
            return;
        };

        if app.views.current_view() != crate::views::ViewID::Game {
            return;
        }

        if !entry.options.allow_bg_interactions {
            let screen_rect = ctx.screen_rect();
            Area::new(Id::new("dialogue_backdrop"))
                .order(Order::Foreground)
                .fixed_pos(screen_rect.min)
                .interactable(false)
                .show(ctx, |ui| {
                    let painter = ui.painter();
                    painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(120));
                    ui.allocate_rect(screen_rect, Sense::click());
                });
        }

        let align = match entry.options.position {
            DialoguePosition::Center => Align2::CENTER_CENTER,
            DialoguePosition::Top => Align2::CENTER_TOP,
            DialoguePosition::Bottom => Align2::CENTER_BOTTOM,
            DialoguePosition::Left => Align2::LEFT_CENTER,
            DialoguePosition::Right => Align2::RIGHT_CENTER,
            DialoguePosition::TopLeft => Align2::LEFT_TOP,
            DialoguePosition::TopRight => Align2::RIGHT_TOP,
            DialoguePosition::BottomLeft => Align2::LEFT_BOTTOM,
            DialoguePosition::BottomRight => Align2::RIGHT_BOTTOM,
        };

        Area::new(Id::new("dialogue_box"))
            .order(Order::Tooltip)
            .anchor(align, Vec2::ZERO)
            .show(ctx, |ui| {
                DialogueBoxComponent::new(&app.game, &mut app.textures, entry).ui(ui);
            });
    }
}
