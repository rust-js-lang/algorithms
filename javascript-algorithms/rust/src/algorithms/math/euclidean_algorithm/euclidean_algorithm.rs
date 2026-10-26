/// The greatest common divisor of `a` and `b`, by Euclid's remainders.
pub fn euclidean_algorithm(a: f64, b: f64) -> f64 {
    let (a, b) = (a.abs(), b.abs());
    if b == 0.0 { a } else { euclidean_algorithm(b, a % b) }
}

js::export_default!(euclidean_algorithm);
