#[derive(Debug)]
pub struct Config {
    pub ticks_per_second: u64,
    /// Higher values will make the monsters exponentially harder to slay depending on their power
    pub slay_duration_power_exponent: f32,
    pub fusion_candidates_min_power_factor: f32,
    pub fusion_candidates_max_power_factor: f32,
    /// Higher values make fused creatures have worse stats compared to fused pair and the power difference
    pub fusion_power_ratio_exp_bias: f32,
}

pub static CONFIG: Config = Config {
    ticks_per_second: 1,
    slay_duration_power_exponent: 2.0,
    fusion_candidates_min_power_factor: 0.5,
    fusion_candidates_max_power_factor: 2.0,
    fusion_power_ratio_exp_bias: 0.1,
};
