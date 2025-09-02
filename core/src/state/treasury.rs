use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Treasury {
    coins: u128,
}

impl Treasury {
    pub fn coins(&self) -> u128 {
        self.coins
    }

    pub fn has_coins(&self, coins: u128) -> bool {
        self.coins >= coins
    }

    pub fn add_coins(&mut self, coins: u128) {
        self.coins = self.coins.saturating_add(coins);
    }

    pub fn remove_coins(&mut self, coins: u128) -> bool {
        if !self.has_coins(coins) {
            return false;
        }
        self.coins = self.coins.saturating_sub(coins);
        true
    }
}
