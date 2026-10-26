/// How many bits `number` takes, its highest set bit's position plus one.
pub fn bit_length(number: i32) -> u32 {
    let mut bits_counter = 0;
    while (1 << bits_counter) <= number {
        bits_counter += 1;
    }
    bits_counter
}

js::export_default!(bit_length);
