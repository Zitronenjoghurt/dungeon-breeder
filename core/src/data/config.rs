#[derive(Debug)]
pub struct Config {
    pub fusion_candidates_min_power_factor: f32,
    pub fusion_candidates_max_power_factor: f32,
    /// Higher values make fused creatures have worse stats compared to fused pair and the power difference
    pub fusion_power_ratio_exp_bias: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fusion_candidates_min_power_factor: 0.5,
            fusion_candidates_max_power_factor: 2.0,
            fusion_power_ratio_exp_bias: 0.1,
        }
    }
}
