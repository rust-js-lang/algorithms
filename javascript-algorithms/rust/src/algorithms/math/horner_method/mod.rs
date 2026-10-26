pub mod classic_polynome;

/// The polynomial of `coefficients`, `[4, 3, 2]` for 4x² + 3x + 2, at
/// `x_val`, by Horner's rule.
pub fn horner_method(coefficients: &[f64], x_val: f64) -> f64 {
    coefficients
        .iter()
        .fold(0.0, |accumulator, current_coefficient| {
            accumulator * x_val + current_coefficient
        })
}

js::export_default!(horner_method);
