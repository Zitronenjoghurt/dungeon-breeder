use crate::data::flags::GameFlag;
use bitvec::prelude::BitVec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameFlags {
    flags: BitVec,
}

impl GameFlags {
    pub fn get(&self, flag: GameFlag) -> bool {
        self.flags.get(flag as usize).map(|b| *b).unwrap_or(false)
    }

    pub fn set_value(&mut self, flag: GameFlag, value: bool) {
        let index = flag as usize;
        if self.flags.len() <= index {
            self.flags.resize(index + 1, false);
        }
        self.flags.set(flag as usize, value);
    }

    pub fn set(&mut self, flag: GameFlag) {
        self.set_value(flag, true);
    }

    pub fn unset(&mut self, flag: GameFlag) {
        self.set_value(flag, false);
    }
}
