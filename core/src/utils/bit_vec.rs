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

    pub fn set_value(&mut self, index: usize, value: bool) {
        if value {
            self.set(index);
        } else {
            self.unset(index);
        }
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

    pub fn iter_set(&self) -> impl Iterator<Item = usize> + '_ {
        (0..self.0.len() * 8).filter(move |&index| self.get(index))
    }

    pub fn iter_xor<'a>(&'a self, other: &'a BitVec) -> impl Iterator<Item = usize> + 'a {
        let max_len = self.0.len().max(other.0.len());
        (0..max_len * 8).filter(move |&index| self.get(index) ^ other.get(index))
    }
}
