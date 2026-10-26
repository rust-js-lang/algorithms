/// Whether `number` is above zero.
pub fn is_positive(number: i32) -> bool {
    // Zero is neither a positive nor a negative number.
    if number == 0 {
        return false;
    }

    // The most significant 32nd bit can be used to determine whether the number is positive.
    ((number >> 31) & 1) == 0
}

js::export_default!(is_positive);
