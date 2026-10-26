use super::get_bit::get_bit;

/// `a + b`, a bit at a time, as a full adder adds them.
pub fn full_adder(a: i32, b: i32) -> i32 {
    let mut result = 0;
    let mut carry = 0;

    for i in 0..32 {
        let ai = get_bit(a, i);
        let bi = get_bit(b, i);
        let carry_in = carry;

        // Calculate binary Ai + Bi without carry (half adder)
        // See Table(1) rows 1 - 4: Si = Ai ^ Bi
        let ai_plus_bi = ai ^ bi;

        // Calculate ith bit of the result by adding the carry bit to Ai + Bi
        // For Table(1) rows 5 - 8 carryIn = 1: Si = Ai ^ Bi ^ 1, flip the bit
        // Fpr Table(1) rows 1 - 4 carryIn = 0: Si = Ai ^ Bi ^ 0, a no-op.
        let bit_sum = ai_plus_bi ^ carry_in;

        // Carry out one to the next most-significant stage
        // when at least one of these is true:
        // 1) Table(1) rows 6, 7: one of Ai OR Bi is 1 AND carryIn = 1
        // 2) Table(1) rows 4, 8: Both Ai AND Bi are 1
        let carry_out = (ai_plus_bi & carry_in) | (ai & bi);
        carry = carry_out;

        // Set ith least significant bit of the result to bitSum.
        result |= bit_sum << i;
    }

    result
}

js::export_default!(full_adder);
