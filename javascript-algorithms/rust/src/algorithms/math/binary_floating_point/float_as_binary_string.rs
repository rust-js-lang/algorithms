/// `float_number`'s bits as a 64-bit floating point number, the sign's
/// first.
pub fn float_as64_binary_string(float_number: f64) -> String {
    format!("{:064b}", float_number.to_bits())
}

/// `float_number`'s bits as a 32-bit floating point number, the sign's
/// first.
pub fn float_as32_binary_string(float_number: f64) -> String {
    format!("{:032b}", (float_number as f32).to_bits())
}
