use crate::data::dialogue::id::DialogueID;
use crate::data::flags::GameFlag;
use crate::state::flags::GameFlags;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum DialogueEvent {
    /// Will end the dialogue
    End,
    /// Will stop the event queue, all following events will be ignored
    Stop,
    /// Will immediately do a relative jump with the given offset
    Jump(i16),
    /// Will set the given flag
    SetFlag(GameFlag),
    /// Will unset the given flag
    Unset(GameFlag),
    /// Will skip the next n events if the flag is set
    SkipIf((GameFlag, u8)),
    /// Will skip the next n events if the flag is not set
    SkipIfNot((GameFlag, u8)),
    /// Will immediately trigger the given dialogue
    TriggerDialogue(DialogueID),
}

impl DialogueEvent {
    pub fn should_ignore_following_events(&self) -> bool {
        matches!(self, DialogueEvent::Stop)
            | matches!(self, DialogueEvent::Jump(_))
            | matches!(self, DialogueEvent::End)
            | matches!(self, DialogueEvent::TriggerDialogue(_))
    }

    pub fn count_events_skipped(&self, flags: &GameFlags) -> u8 {
        match self {
            DialogueEvent::SkipIf((flag, amount)) => {
                if flags.get(*flag) {
                    *amount
                } else {
                    0
                }
            }
            DialogueEvent::SkipIfNot((flag, amount)) => {
                if !flags.get(*flag) {
                    *amount
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
}

#[macro_export]
macro_rules! dialogue_event {
    (end) => {
        $crate::data::dialogue::event::DialogueEvent::End
    };

    (stop) => {
        $crate::data::dialogue::event::DialogueEvent::Stop
    };

    (step) => {
        $crate::data::dialogue::event::DialogueEvent::Jump(1)
    };

    (jump: $offset:expr) => {
        $crate::data::dialogue::event::DialogueEvent::Jump($offset)
    };

    (set: $flag:expr) => {
        $crate::data::dialogue::event::DialogueEvent::SetFlag($flag)
    };

    (unset: $flag:expr) => {
        $crate::data::dialogue::event::DialogueEvent::Unset($flag)
    };

    (skip_if: $flag:expr) => {
        $crate::data::dialogue::event::DialogueEvent::SkipIf(($flag, 1))
    };

    (skip_if_not: $flag:expr) => {
        $crate::data::dialogue::event::DialogueEvent::SkipIfNot(($flag, 1))
    };

    (skip_n_if: ($flag:expr, $n:expr)) => {
        $crate::data::dialogue::event::DialogueEvent::SkipIf(($flag, $n))
    };

    (skip_n_if_not: ($flag:expr, $n:expr)) => {
        $crate::data::dialogue::event::DialogueEvent::SkipIf(($flag, $n))
    };
}
