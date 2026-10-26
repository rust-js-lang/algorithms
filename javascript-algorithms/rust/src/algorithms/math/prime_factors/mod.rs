/// The prime factors of `n`, from the least.
pub fn prime_factors(n: f64) -> Vec<f64> {
    // Clone n to avoid function arguments override.
    let mut nn = n;

    // Array that stores the all the prime factors.
    let mut factors = Vec::new();

    // Running the loop till sqrt(n) instead of n to optimise time complexity from O(n) to O(sqrt(n)).
    let mut factor = 2.0;
    while factor <= nn.sqrt() {
        // Check that factor divides n without a reminder.
        while nn % factor == 0.0 {
            // Overriding the value of n.
            nn /= factor;
            // Saving the factor.
            factors.push(factor);
        }
        factor += 1.0;
    }

    // The ultimate reminder should be a last prime factor,
    // unless it is not 1 (since 1 is not a prime number).
    if nn != 1.0 {
        factors.push(nn);
    }

    factors
}

/// Hardy-Ramanujan approximation of prime factors count.
pub fn hardy_ramanujan(n: f64) -> f64 {
    n.ln().ln()
}
