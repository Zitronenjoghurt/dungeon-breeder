#[derive(Debug)]
pub enum DialogueEvent {
    Jump(i16),
}

#[macro_export]
macro_rules! dialogue_event {
    (step) => {
        $crate::data::dialogue::event::DialogueEvent::Jump(1)
    };

    (jump: $offset:expr) => {
        $crate::data::dialogue::event::DialogueEvent::Jump($offset)
    };
}
