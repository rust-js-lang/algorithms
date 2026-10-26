use super::divide_by_two::divide_by_two;
use super::is_even::is_even;
use super::is_positive::is_positive;
use super::multiply_by_two::multiply_by_two;

/// `a * b`, by doubling `a` and halving `b`.
pub fn multiply(a: i32, b: i32) -> i32 {
    // If a is zero or b is zero or if both a and b are zeros then the production is also zero.
    if b == 0 || a == 0 {
        return 0;
    }

    // Otherwise we will have four different cases that are described above.
    let multiply_by_odd_positive = || multiply(multiply_by_two(a), divide_by_two(b - 1)) + a;
    let multiply_by_odd_negative = || multiply(multiply_by_two(a), divide_by_two(b + 1)) - a;

    let multiply_by_even = || multiply(multiply_by_two(a), divide_by_two(b));
    let multiply_by_odd = || {
        if is_positive(b) {
            multiply_by_odd_positive()
        } else {
            multiply_by_odd_negative()
        }
    };

    if is_even(b) {
        multiply_by_even()
    } else {
        multiply_by_odd()
    }
}

js::export_default!(multiply);
