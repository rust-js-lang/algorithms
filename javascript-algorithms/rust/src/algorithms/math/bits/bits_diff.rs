use super::count_set_bits::count_set_bits;

/// How many bits differ between `number_a` and `number_b`.
pub fn bits_diff(number_a: i32, number_b: i32) -> u32 {
    count_set_bits(number_a ^ number_b)
}

js::export_default!(bits_diff);
