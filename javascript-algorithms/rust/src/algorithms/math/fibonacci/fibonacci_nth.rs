/// The `n`th Fibonacci number, from 1.
pub fn fibonacci_nth(n: u32) -> f64 {
    let (mut current, mut previous) = (1.0, 0.0);
    for _ in 1..n {
        current += previous;
        previous = current - previous;
    }
    current
}

js::export_default!(fibonacci_nth);
