use crate::data::item::id::ItemID;
use rand::Rng;
use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct CreatureItemDrop {
    pub item_id: ItemID,
    pub min_proficiency: f32,
    pub drop_chance: f32,
    pub count_range: &'static RangeInclusive<u32>,
}

impl CreatureItemDrop {
    pub fn generate_drop(&self, rng: &mut impl Rng, proficiency: f32) -> Option<u32> {
        if proficiency < self.min_proficiency {
            return None;
        }

        let chance_roll = rng.random_range(0.0..1.0);
        if chance_roll > self.drop_chance {
            return None;
        }

        let count = rng.random_range(self.count_range.clone());
        if count == 0 { None } else { Some(count) }
    }
}
