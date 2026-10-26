/// `number` with its bit at `bit_position` set.
pub fn set_bit(number: i32, bit_position: u32) -> i32 {
    number | (1 << bit_position)
}

js::export_default!(set_bit);
