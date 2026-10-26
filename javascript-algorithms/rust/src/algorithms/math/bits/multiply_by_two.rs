/// `number` doubled, by shifting it left a bit.
pub fn multiply_by_two(number: i32) -> i32 {
    number << 1
}

js::export_default!(multiply_by_two);
