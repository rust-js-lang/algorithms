/// Whether `number` is prime: no odd divider up to its square root divides it.
pub fn trial_division(number: f64) -> bool {
    // A number that isn't an integer isn't prime, nor is one of 1 or less.
    if number % 1.0 != 0.0 || number <= 1.0 {
        return false;
    }
    if number <= 3.0 {
        return true;
    }
    if number % 2.0 == 0.0 {
        return false;
    }
    let divider_limit = number.sqrt();
    let mut divider = 3.0;
    while divider <= divider_limit {
        if number % divider == 0.0 {
            return false;
        }
        divider += 2.0;
    }
    true
}

js::export_default!(trial_division);
