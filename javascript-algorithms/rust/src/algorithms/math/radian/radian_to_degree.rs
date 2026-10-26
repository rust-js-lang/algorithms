/// `radian` in degrees.
pub fn radian_to_degree(radian: f64) -> f64 {
    radian * (180.0 / std::f64::consts::PI)
}

js::export_default!(radian_to_degree);
