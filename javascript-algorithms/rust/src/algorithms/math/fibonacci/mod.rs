pub mod fibonacci_nth;
pub mod fibonacci_nth_closed_form;

/// The first `n` Fibonacci numbers, from 1.
pub fn fibonacci(n: u32) -> Vec<f64> {
    let mut sequence = vec![1.0];
    let (mut current, mut previous) = (1.0, 0.0);
    for _ in 1..n {
        current += previous;
        previous = current - previous;
        sequence.push(current);
    }
    sequence
}

js::export_default!(fibonacci);
