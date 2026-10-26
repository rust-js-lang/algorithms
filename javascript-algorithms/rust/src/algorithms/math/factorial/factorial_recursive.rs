/// `number!`, recursively.
pub fn factorial_recursive(number: f64) -> f64 {
    if number > 1.0 {
        number * factorial_recursive(number - 1.0)
    } else {
        1.0
    }
}

js::export_default!(factorial_recursive);
