/// `number` with its bit at `bit_position` cleared.
pub fn clear_bit(number: i32, bit_position: u32) -> i32 {
    let mask = !(1 << bit_position);
    number & mask
}

js::export_default!(clear_bit);
