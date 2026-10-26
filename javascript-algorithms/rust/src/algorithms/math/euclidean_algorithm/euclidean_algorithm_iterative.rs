/// The greatest common divisor of `a` and `b`, by subtracting the smaller
/// from the larger till they're equal, or one is zero.
pub fn euclidean_algorithm_iterative(a: f64, b: f64) -> f64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while a != 0.0 && b != 0.0 && a != b {
        (a, b) = if a > b { (a - b, b) } else { (a, b - a) };
    }
    if a != 0.0 { a } else { b }
}

js::export_default!(euclidean_algorithm_iterative);
