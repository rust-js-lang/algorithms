/// `number` halved, by shifting it right a bit.
pub fn divide_by_two(number: i32) -> i32 {
    number >> 1
}

js::export_default!(divide_by_two);
