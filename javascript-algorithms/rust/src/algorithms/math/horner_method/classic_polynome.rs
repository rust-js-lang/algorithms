/// The polynomial of `coefficients`, `[4, 3, 2]` for 4x² + 3x + 2, at
/// `x_val`, each term by its power; `coefficients` left reversed.
pub fn classic_polynome(coefficients: &mut [f64], x_val: f64) -> f64 {
    coefficients.reverse();
    coefficients
        .iter()
        .enumerate()
        .fold(0.0, |accumulator, (index, current_coefficient)| {
            accumulator + current_coefficient * x_val.powf(index as f64)
        })
}

js::export_default!(classic_polynome);
