/// `number1 * number2`, adding `number1` shifted by each set bit of
/// `number2`.
pub fn multiply_unsigned(number1: i32, number2: i32) -> f64 {
    let mut result = 0.0;

    // Let's treat number2 as a multiplier for the number1.
    let mut multiplier = number2;

    // Multiplier current bit index.
    let mut bit_index = 0;

    // Go through all bits of number2.
    while multiplier != 0 {
        // Check if current multiplier bit is set.
        if multiplier & 1 != 0 {
            // In case if multiplier's bit at position bitIndex is set
            // it would mean that we need to multiply number1 by the power
            // of bit with index bitIndex and then add it to the result.
            result += f64::from(number1 << bit_index);
        }

        bit_index += 1;
        multiplier >>= 1;
    }

    result
}

js::export_default!(multiply_unsigned);
