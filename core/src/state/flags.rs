use crate::data::flags::GameFlag;
use crate::utils::bit_vec::BitVec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameFlags {
    flags: BitVec,
}

impl GameFlags {
    pub fn get(&self, flag: GameFlag) -> bool {
        self.flags.get(flag as usize)
    }

    pub fn set_value(&mut self, flag: GameFlag, value: bool) {
        if value {
            self.set(flag);
        } else {
            self.unset(flag);
        }
    }

    pub fn set(&mut self, flag: GameFlag) {
        self.flags.set(flag as usize);
    }

    pub fn unset(&mut self, flag: GameFlag) {
        self.flags.unset(flag as usize);
    }
}
