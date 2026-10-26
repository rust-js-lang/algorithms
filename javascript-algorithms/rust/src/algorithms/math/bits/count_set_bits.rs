/// How many of `original_number`'s bits are set.
pub fn count_set_bits(original_number: i32) -> u32 {
    let mut set_bits_count = 0;
    let mut number = original_number as u32;
    while number != 0 {
        // Add last bit of the number to the sum of set bits.
        set_bits_count += number & 1;
        // Shift number right by one bit to investigate other bits.
        number >>= 1;
    }
    set_bits_count
}

js::export_default!(count_set_bits);
