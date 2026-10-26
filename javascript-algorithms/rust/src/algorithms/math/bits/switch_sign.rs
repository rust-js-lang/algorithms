/// `-number`, by two's complement: its bits flipped, plus one.
pub fn switch_sign(number: i32) -> i32 {
    !number + 1
}

js::export_default!(switch_sign);
