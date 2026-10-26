/// `degree` in radians.
pub fn degree_to_radian(degree: f64) -> f64 {
    degree * (std::f64::consts::PI / 180.0)
}

js::export_default!(degree_to_radian);
