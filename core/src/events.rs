use crate::events::event::GameEvent;
use std::cell::RefCell;

mod event;

#[derive(Debug, Default)]
pub struct GameEvents {
    queue: RefCell<Vec<GameEvent>>,
}

impl GameEvents {
    pub fn take_events(&self) -> Vec<GameEvent> {
        if let Ok(mut queue) = self.queue.try_borrow_mut() {
            queue.drain(..).collect()
        } else {
            vec![]
        }
    }

    pub fn push_event(&self, event: GameEvent) {
        if let Ok(mut queue) = self.queue.try_borrow_mut() {
            queue.push(event);
        }
    }
}
