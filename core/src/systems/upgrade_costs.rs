use crate::data::config::CONFIG;

pub fn dungeon_layer_unlock_cost(layer: usize) -> Option<u128> {
    CONFIG.base_dungeon_layer_unlock_costs.get(layer).copied()
}

pub fn dungeon_layer_slot_unlock_cost(layer: usize, slot: usize) -> Option<u128> {
    let costs = CONFIG.base_dungeon_layer_slot_unlock_costs.get(slot)?;
    Some(
        costs.pow(
            CONFIG.base_dungeon_layer_slot_unlock_cost_exponent_per_layer * (layer as u32 + 1),
        ),
    )
}
