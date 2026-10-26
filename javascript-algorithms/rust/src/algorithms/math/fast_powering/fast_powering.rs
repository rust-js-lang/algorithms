/// `base` raised to `power`, by squaring the half power.
pub fn fast_powering(base: f64, power: f64) -> f64 {
    if power == 0.0 {
        // Anything that is raised to the power of zero is 1.
        return 1.0;
    }
    let multiplier = fast_powering(base, (power / 2.0).floor());
    if power % 2.0 == 0.0 {
        multiplier * multiplier
    } else {
        multiplier * multiplier * base
    }
}

js::export_default!(fast_powering);
