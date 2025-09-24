use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub enum GameFeedbackNotificationType {
    #[default]
    Info,
    Success,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameFeedbackNotification {
    pub notification_type: GameFeedbackNotificationType,
    pub message: String,
}

impl GameFeedbackNotification {
    pub fn new(
        notification_type: GameFeedbackNotificationType,
        message: impl Into<String>,
    ) -> Self {
        Self {
            notification_type,
            message: message.into(),
        }
    }

    pub fn info(message: impl Into<String>) -> Self {
        Self::new(GameFeedbackNotificationType::Info, message)
    }

    pub fn success(message: impl Into<String>) -> Self {
        Self::new(GameFeedbackNotificationType::Success, message)
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::new(GameFeedbackNotificationType::Error, message)
    }
}
