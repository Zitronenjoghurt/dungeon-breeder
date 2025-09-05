use crate::actions::feedback::GameActionFeedback;
use crate::error::GameError;

#[derive(Debug, Default)]
pub struct GameActionReport {
    pub errors: Vec<GameError>,
    pub feedback: Vec<GameActionFeedback>,
}

impl GameActionReport {
    pub fn on_error(&mut self, error: GameError) {
        self.errors.push(error);
    }

    pub fn on_feedback(&mut self, feedback: GameActionFeedback) {
        self.feedback.push(feedback);
    }
}
