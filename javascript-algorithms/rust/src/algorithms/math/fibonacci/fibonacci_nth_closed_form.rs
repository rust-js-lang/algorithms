/// The Fibonacci number at `position`, by Binet's formula, which holds up to
/// the 70th.
pub fn fibonacci_closed_form(position: f64) -> f64 {
    const TOP_MAX_VALID_POSITION: f64 = 70.0;
    if !(1.0..=TOP_MAX_VALID_POSITION).contains(&position) {
        panic!("Can't handle position smaller than 1 or greater than {TOP_MAX_VALID_POSITION}");
    }
    let sqrt5 = 5f64.sqrt();
    let phi = (1.0 + sqrt5) / 2.0;
    (phi.powf(position) / sqrt5 + 0.5).floor()
}

js::export_default!(fibonacci_closed_form);
