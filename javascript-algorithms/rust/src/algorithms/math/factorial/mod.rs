pub mod factorial_recursive;

/// `number!`, each integer from 2 to it multiplied in.
pub fn factorial(number: f64) -> f64 {
    let mut result = 1.0;
    let mut i = 2.0;
    while i <= number {
        result *= i;
        i += 1.0;
    }
    result
}

js::export_default!(factorial);
