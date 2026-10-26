/// Whether `number` is a power of two, or zero: no bit but one set.
pub fn is_power_of_two(number: i32) -> bool {
    (number & (number - 1)) == 0
}

js::export_default!(is_power_of_two);
