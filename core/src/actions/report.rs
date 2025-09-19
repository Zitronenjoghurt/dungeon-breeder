use crate::error::GameError;

#[derive(Debug, Default)]
pub struct GameActionReport {
    pub errors: Vec<GameError>,
}

impl GameActionReport {
    pub fn on_error(&mut self, error: GameError) {
        self.errors.push(error);
    }
}
