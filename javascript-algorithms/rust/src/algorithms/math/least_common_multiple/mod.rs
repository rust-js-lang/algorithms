use crate::algorithms::math::euclidean_algorithm::euclidean_algorithm;

/// The least common multiple of `a` and `b`: their product over their
/// greatest common divisor.
pub fn least_common_multiple(a: f64, b: f64) -> f64 {
    if a == 0.0 || b == 0.0 {
        0.0
    } else {
        (a * b).abs() / euclidean_algorithm(a, b)
    }
}

js::export_default!(least_common_multiple);
