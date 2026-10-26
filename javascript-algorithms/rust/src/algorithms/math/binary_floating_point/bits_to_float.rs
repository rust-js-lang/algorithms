/// How many bits each part of a floating point number takes.
struct PrecisionConfig {
    sign_bits_count: usize,
    exponent_bits_count: usize,
}

/// @see: https://en.wikipedia.org/wiki/Half-precision_floating-point_format
const HALF: PrecisionConfig = PrecisionConfig {
    sign_bits_count: 1,
    exponent_bits_count: 5,
};

/// @see: https://en.wikipedia.org/wiki/Single-precision_floating-point_format
const SINGLE: PrecisionConfig = PrecisionConfig {
    sign_bits_count: 1,
    exponent_bits_count: 8,
};

/// @see: https://en.wikipedia.org/wiki/Double-precision_floating-point_format
const DOUBLE: PrecisionConfig = PrecisionConfig {
    sign_bits_count: 1,
    exponent_bits_count: 11,
};

/// The number `bits`, each 0 or 1, are, of `precision_config`'s parts.
fn bits_to_float(bits: &[f64], precision_config: &PrecisionConfig) -> f64 {
    let PrecisionConfig {
        sign_bits_count,
        exponent_bits_count,
    } = *precision_config;

    // Figuring out the sign.
    let sign = (-1f64).powf(bits[0]); // -1^1 = -1, -1^0 = 1

    // Calculating the exponent value.
    let exponent_bias = 2f64.powf((exponent_bits_count - 1) as f64) - 1.0;
    let exponent_bits = &bits[sign_bits_count..sign_bits_count + exponent_bits_count];
    let exponent_unbiased =
        exponent_bits
            .iter()
            .enumerate()
            .fold(0.0, |exponent_so_far, (bit_index, current_bit)| {
                let bit_power_of_two = 2f64.powf((exponent_bits_count - bit_index - 1) as f64);
                exponent_so_far + current_bit * bit_power_of_two
            });
    let exponent = exponent_unbiased - exponent_bias;

    // Calculating the fraction value.
    let fraction_bits = &bits[sign_bits_count + exponent_bits_count..];
    let fraction =
        fraction_bits
            .iter()
            .enumerate()
            .fold(0.0, |fraction_so_far, (bit_index, current_bit)| {
                let bit_power_of_two = 2f64.powf(-((bit_index + 1) as f64));
                fraction_so_far + current_bit * bit_power_of_two
            });

    // Putting all parts together to calculate the final number.
    sign * 2f64.powf(exponent) * (1.0 + fraction)
}

/// The 16-bit floating point number `bits` are.
pub fn bits_to_float16(bits: &[f64]) -> f64 {
    bits_to_float(bits, &HALF)
}

/// The 32-bit floating point number `bits` are.
pub fn bits_to_float32(bits: &[f64]) -> f64 {
    bits_to_float(bits, &SINGLE)
}

/// The 64-bit floating point number `bits` are.
pub fn bits_to_float64(bits: &[f64]) -> f64 {
    bits_to_float(bits, &DOUBLE)
}
