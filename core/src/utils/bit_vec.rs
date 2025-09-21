use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BitVec(Vec<u8>);

impl BitVec {
    pub fn get(&self, index: usize) -> bool {
        let byte_index = index / 8;
        let bit_index = index % 8;
        self.0
            .get(byte_index)
            .map(|byte| (byte >> bit_index) & 1 == 1)
            .unwrap_or(false)
    }

    pub fn set(&mut self, index: usize) {
        let byte_index = index / 8;
        let bit_index = index % 8;
        if byte_index >= self.0.len() {
            self.0.resize(byte_index + 1, 0);
        }
        self.0[byte_index] |= 1 << bit_index;
    }

    pub fn unset(&mut self, index: usize) {
        let byte_index = index / 8;
        let bit_index = index % 8;
        if byte_index >= self.0.len() {
            self.0.resize(byte_index + 1, 0);
        }
        self.0[byte_index] &= !(1 << bit_index);
    }
}
