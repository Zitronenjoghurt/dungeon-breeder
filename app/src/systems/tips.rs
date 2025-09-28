use crate::data::tip::Tip;
use dungeon_breeder_core::utils::bit_vec::BitVec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TipsSystem {
    pub show: BitVec,
    pub read: BitVec,
    pub selected_tip: Option<Tip>,
    pub is_window_open: bool,
    pub show_all: bool,
}

impl TipsSystem {
    pub fn tips_relevant(&self) -> impl Iterator<Item = Tip> {
        self.show
            .iter_xor(&self.read)
            .filter_map(|index| Tip::from_repr(index as u16))
    }

    pub fn tips_all(&self) -> impl Iterator<Item = Tip> {
        self.show
            .iter_set()
            .filter_map(|index| Tip::from_repr(index as u16))
    }

    pub fn show_tip(&mut self, tip: Tip) {
        let is_set = self.show.get(tip as usize);
        if is_set {
            return;
        }
        self.show.set(tip as usize);
        self.is_window_open = true;
    }

    pub fn read_tip(&mut self, tip: Tip) {
        self.read.set(tip as usize);
    }

    pub fn has_read_tip(&self, tip: Tip) -> bool {
        self.read.get(tip as usize)
    }
}
