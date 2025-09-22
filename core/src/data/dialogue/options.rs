use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub enum DialoguePosition {
    #[default]
    Center,
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct DialogueOptions {
    pub allow_bg_interactions: bool,
    pub position: DialoguePosition,
}

impl DialogueOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_position(position: DialoguePosition) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }

    pub fn from_allow_bg() -> Self {
        Self {
            allow_bg_interactions: true,
            ..Default::default()
        }
    }

    pub fn position(mut self, position: DialoguePosition) -> Self {
        self.position = position;
        self
    }

    pub fn allow_bg(mut self) -> Self {
        self.allow_bg_interactions = true;
        self
    }
}
