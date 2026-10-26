/// Whether `number` is a power of two: it halves to 1 without a remainder.
pub fn is_power_of_two(number: f64) -> bool {
    // 1 (2^0) is the smallest power of two.
    if number < 1.0 {
        return false;
    }
    let mut divided = number;
    while divided != 1.0 {
        if divided % 2.0 != 0.0 {
            return false;
        }
        divided /= 2.0;
    }
    true
}

js::export_default!(is_power_of_two);
