pub fn is_almost_zero(x: f32) -> bool {
    x.abs() < f32::EPSILON
}
