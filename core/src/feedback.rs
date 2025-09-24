use serde::{Deserialize, Serialize};

pub mod notification;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameFeedback {
    CloseApp,
    Notification(notification::GameFeedbackNotification),
}
