use rand::prelude::Distribution;
use rand_distr::Normal;

const MEAN: f32 = 0.5;
const STD_DEV: f32 = 1.0 / 6.0;

pub fn random_normalized() -> f32 {
    let mut rng = rand::rng();
    let normal = Normal::new(MEAN, STD_DEV).unwrap();
    let value: f32 = normal.sample(&mut rng);
    value.clamp(0.0, 1.0)
}

pub fn random_normalized_fusion(a: f32, b: f32) -> f32 {
    ((a * random_normalized()) + (b * random_normalized())).clamp(0.0, 1.0)
}
