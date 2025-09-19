use crate::events::event::GameEvent;

pub mod event;

#[derive(Debug, Default)]
pub struct GameEvents {
    queue: Vec<GameEvent>,
}

impl GameEvents {
    pub fn take_events(&mut self) -> Vec<GameEvent> {
        self.queue.drain(..).collect()
    }

    pub fn push_event(&mut self, event: GameEvent) {
        self.queue.push(event);
    }
}
