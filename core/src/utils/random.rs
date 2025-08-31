use rand::prelude::Distribution;
use rand::Rng;
use rand_distr::Normal;

const MEAN: f32 = 0.5;
const STD_DEV: f32 = 1.0 / 6.0;

pub fn random_normal() -> f32 {
    let mut rng = rand::rng();
    let normal = Normal::new(MEAN, STD_DEV).unwrap();
    let value: f32 = normal.sample(&mut rng);
    value.clamp(0.0, 1.0)
}

pub fn random_normal_exp_bias(exp: f32) -> f32 {
    random_normal().powf(exp).clamp(0.0, 1.0)
}

pub fn random_normalized() -> f32 {
    let mut rng = rand::rng();
    rng.random_range(0.0..=1.0)
}

pub fn random_normal_combination(a: f32, b: f32) -> f32 {
    (a * random_normal()) + (b * random_normal())
}

pub fn random_normal_combination_01(a: f32, b: f32) -> f32 {
    ((a * random_normal()) + (b * random_normal())).clamp(0.0, 1.0)
}
