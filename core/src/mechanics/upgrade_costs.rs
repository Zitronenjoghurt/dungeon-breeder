use crate::data::config::CONFIG;

pub fn dungeon_layer_unlock_cost(layer: usize) -> Option<u128> {
    CONFIG.base_dungeon_layer_unlock_costs.get(layer).copied()
}

pub fn dungeon_layer_slot_unlock_cost(layer: usize, slot: usize) -> Option<u128> {
    let costs = CONFIG.base_dungeon_layer_slot_unlock_costs.get(slot)?;
    if layer == 0 {
        return Some(*costs);
    }

    Some((*costs as f32).powf(
        CONFIG.base_dungeon_layer_slot_unlock_cost_exponent_per_layer * (layer as u32 + 1) as f32,
    ) as u128)
}
