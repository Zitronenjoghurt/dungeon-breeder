use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum_macros::EnumIter;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, EnumIter,
)]
pub enum UIScale {
    X0_25,
    X0_5,
    X0_75,
    X1_0,
    X1_25,
    X1_5,
    #[default]
    X1_75,
    X2_0,
    X2_25,
    X2_5,
    X2_75,
    X3_0,
}

impl UIScale {
    pub fn as_f32(self) -> f32 {
        match self {
            Self::X0_25 => 0.25,
            Self::X0_5 => 0.5,
            Self::X0_75 => 0.75,
            Self::X1_0 => 1.0,
            Self::X1_25 => 1.25,
            Self::X1_5 => 1.5,
            Self::X1_75 => 1.75,
            Self::X2_0 => 2.0,
            Self::X2_25 => 2.25,
            Self::X2_5 => 2.5,
            Self::X2_75 => 2.75,
            Self::X3_0 => 3.0,
        }
    }
}

impl Display for UIScale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X0_25 => write!(f, "x0.25"),
            Self::X0_5 => write!(f, "x0.5"),
            Self::X0_75 => write!(f, "x0.75"),
            Self::X1_0 => write!(f, "x1.0"),
            Self::X1_25 => write!(f, "x1.25"),
            Self::X1_5 => write!(f, "x1.5"),
            Self::X1_75 => write!(f, "x1.75"),
            Self::X2_0 => write!(f, "x2.0"),
            Self::X2_25 => write!(f, "x2.25"),
            Self::X2_5 => write!(f, "x2.5"),
            Self::X2_75 => write!(f, "x2.75"),
            Self::X3_0 => write!(f, "x3.0"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsSystem {
    ui_scale: UIScale,
    #[serde(skip, default = "default_true")]
    dirty: bool,
}

impl Default for SettingsSystem {
    fn default() -> Self {
        Self {
            ui_scale: UIScale::default(),
            dirty: true,
        }
    }
}

impl SettingsSystem {
    pub fn update(&mut self, ctx: &egui::Context) {
        if !self.dirty {
            return;
        }

        ctx.set_pixels_per_point(self.ui_scale.as_f32());
        self.dirty = false;
    }

    pub fn get_ui_scale(&self) -> UIScale {
        self.ui_scale
    }

    pub fn set_ui_scale(&mut self, ui_scale: UIScale) {
        if self.ui_scale == ui_scale {
            return;
        }
        self.ui_scale = ui_scale;
        self.dirty = true;
    }
}

fn default_true() -> bool {
    true
}
