use crate::data::config::styles::{ConfigStyles, CONFIG_STYLES};

pub mod styles;

#[derive(Debug)]
pub struct Config {
    /// How many generations of followup events will be processed within the same tick.
    pub max_event_generations: u8,
    /// Higher values will make the monsters exponentially harder to slay depending on their power
    pub slay_duration_power_exponent: f32,
    /// Higher values will make slaying take longer
    pub slay_duration_base_bias_factor: f32,
    /// Higher values will make the monster take exponentially longer to regenerate after being slain
    pub regeneration_duration_power_exponent: f32,
    /// How much the regeneration stat will affect the regeneration duration
    pub regeneration_duration_regeneration_factor_min: f32,
    pub regeneration_duration_regeneration_factor_max: f32,
    /// Higher values will make regeneration take longer
    pub regeneration_duration_base_bias_factor: f32,
    /// Higher values will make breeding bias more towards the higher stats
    pub breeding_base_bias_factor: f32,
    /// Higher values will make the monster take exponentially longer to be available for breeding again
    pub breeding_duration_exponent: f32,
    /// How much fertility will affect the breeding duration
    pub breeding_duration_fertility_factor_min: f32,
    pub breeding_duration_fertility_factor_max: f32,
    pub fusion_candidates_min_power_factor: f32,
    pub fusion_candidates_max_power_factor: f32,
    /// Higher values make fused creatures have worse stats compared to fused pair and the power difference
    pub fusion_power_ratio_exp_bias: f32,
    /// Higher values will make it easier to achieve better fusions
    pub fusion_power_base_bias_factor: f32,
    pub base_dungeon_layer_unlock_costs: &'static [u128],
    pub base_dungeon_layer_slot_unlock_costs: &'static [u128],
    pub base_dungeon_layer_slot_unlock_cost_exponent_per_layer: f32,
    pub styles: &'static ConfigStyles,
}

pub static CONFIG: Config = Config {
    max_event_generations: 10,
    slay_duration_power_exponent: 1.5,
    regeneration_duration_power_exponent: 1.25,
    slay_duration_base_bias_factor: 0.5,
    regeneration_duration_regeneration_factor_min: 0.0,
    regeneration_duration_regeneration_factor_max: 0.5,
    regeneration_duration_base_bias_factor: 0.5,
    breeding_base_bias_factor: 1.1,
    breeding_duration_exponent: 1.0,
    breeding_duration_fertility_factor_min: 0.0,
    breeding_duration_fertility_factor_max: 0.8,
    fusion_candidates_min_power_factor: 0.5,
    fusion_candidates_max_power_factor: 2.0,
    fusion_power_ratio_exp_bias: 0.1,
    fusion_power_base_bias_factor: 1.5,
    base_dungeon_layer_unlock_costs: &[0, 1000, 50000, 1000000],
    base_dungeon_layer_slot_unlock_costs: &[0, 10, 100, 1500, 5000, 25000],
    base_dungeon_layer_slot_unlock_cost_exponent_per_layer: 1.5,
    styles: &CONFIG_STYLES,
};
