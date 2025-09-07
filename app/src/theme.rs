use crate::types::color::ColorConvert;
use dungeon_breeder_core::data::config::styles::*;
use egui::{Context, Visuals};

pub fn apply_glomzy_theme(ctx: &Context) {
    let mut visuals = Visuals::dark();

    visuals.window_fill = COLOR_GRAY_DARKEST.to_egui().linear_multiply(0.95);
    visuals.panel_fill = COLOR_GRAY_DARKEST.to_egui().linear_multiply(0.95);
    visuals.extreme_bg_color = COLOR_BROWN_DARK.to_egui();
    visuals.faint_bg_color = COLOR_GRAY_DARKEST.to_egui().linear_multiply(1.2);
    visuals.code_bg_color = COLOR_BROWN_DARK.to_egui();

    visuals.widgets.noninteractive.fg_stroke.color = COLOR_GRAY_LIGHT.to_egui();
    visuals.widgets.inactive.fg_stroke.color = COLOR_GRAY_LIGHT.to_egui();
    visuals.widgets.hovered.fg_stroke.color = COLOR_GRAY_LIGHT.to_egui();
    visuals.widgets.active.fg_stroke.color = COLOR_GRAY_LIGHT.to_egui();
    visuals.widgets.open.fg_stroke.color = COLOR_GRAY_LIGHT.to_egui();

    visuals.widgets.noninteractive.bg_fill = COLOR_BROWN_DARK.to_egui();
    visuals.widgets.noninteractive.weak_bg_fill = COLOR_BROWN_DARK.to_egui();

    visuals.widgets.inactive.bg_fill = COLOR_PURPLE_DARK.to_egui();
    visuals.widgets.inactive.weak_bg_fill = COLOR_PURPLE_DARK.to_egui();

    visuals.widgets.hovered.bg_fill = COLOR_PURPLE.to_egui();
    visuals.widgets.hovered.weak_bg_fill = COLOR_PURPLE.to_egui();

    visuals.widgets.active.bg_fill = COLOR_GRAY_DARK.to_egui();
    visuals.widgets.active.weak_bg_fill = COLOR_GRAY_DARK.to_egui();

    visuals.widgets.open.bg_fill = COLOR_PURPLE_DARK.to_egui();
    visuals.widgets.open.weak_bg_fill = COLOR_PURPLE_DARK.to_egui();

    let border_stroke = egui::Stroke {
        width: 1.0,
        color: COLOR_GRAY_DARK.to_egui(),
    };

    visuals.widgets.noninteractive.bg_stroke = border_stroke;
    visuals.widgets.inactive.bg_stroke = border_stroke;
    visuals.widgets.hovered.bg_stroke = egui::Stroke {
        width: 1.0,
        color: COLOR_GRAY.to_egui(),
    };
    visuals.widgets.active.bg_stroke = egui::Stroke {
        width: 1.0,
        color: COLOR_PURPLE_LIGHT.to_egui(),
    };
    visuals.widgets.open.bg_stroke = border_stroke;
    visuals.window_stroke = border_stroke;

    visuals.hyperlink_color = COLOR_CYAN_LIGHT.to_egui();
    visuals.error_fg_color = COLOR_RED_LIGHT.to_egui();
    visuals.warn_fg_color = COLOR_ORANGE.to_egui();

    visuals.selection = egui::style::Selection {
        bg_fill: COLOR_PURPLE.to_egui().linear_multiply(1.2),
        stroke: egui::Stroke {
            width: 1.0,
            color: COLOR_GRAY_LIGHT.to_egui(),
        },
    };

    ctx.set_visuals(visuals);
}
