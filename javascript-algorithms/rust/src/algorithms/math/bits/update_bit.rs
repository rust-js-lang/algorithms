/// `number` with its bit at `bit_position` set where `bit_value` is, else
/// cleared.
pub fn update_bit(number: i32, bit_position: u32, bit_value: bool) -> i32 {
    // Normalized bit value.
    let bit_value_normalized = i32::from(bit_value);

    // Init clear mask.
    let clear_mask = !(1 << bit_position);

    // Clear bit value and then set it up to required value.
    (number & clear_mask) | (bit_value_normalized << bit_position)
}

js::export_default!(update_bit);
