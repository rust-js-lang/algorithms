/// Whether `number` is a power of two: it has exactly one bit set.
pub fn is_power_of_two_bitwise(number: i32) -> bool {
    // 1 (2^0) is the smallest power of two.
    if number < 1 {
        return false;
    }
    number & (number - 1) == 0
}

js::export_default!(is_power_of_two_bitwise);
