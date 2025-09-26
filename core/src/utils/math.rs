pub fn f32_interpolate(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}
