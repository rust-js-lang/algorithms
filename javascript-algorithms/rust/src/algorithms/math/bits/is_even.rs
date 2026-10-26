/// Whether `number` is even: its last bit clear.
pub fn is_even(number: i32) -> bool {
    (number & 1) == 0
}

js::export_default!(is_even);
