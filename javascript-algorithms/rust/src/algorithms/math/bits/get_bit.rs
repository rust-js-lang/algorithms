/// `number`'s bit at `bit_position`, 0 or 1.
pub fn get_bit(number: i32, bit_position: u32) -> i32 {
    (number >> bit_position) & 1
}

js::export_default!(get_bit);
