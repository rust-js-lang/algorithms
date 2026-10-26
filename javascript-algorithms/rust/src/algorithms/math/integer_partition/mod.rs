/// The number of ways to write `number` as a sum of positive integers.
pub fn integer_partition(number: usize) -> f64 {
    // Create partition matrix for solving this task using Dynamic Programming.
    // Its first row represents how many ways we would have to combine the
    // numbers 1, 2, 3, ..., n with number 0. We would have zero ways
    // obviously since with zero number we may form only zero.
    let mut partition_matrix = vec![vec![0.0; number + 1]; number + 1];

    // Let's fill the first column. It represents the number of ways we can form
    // number zero out of numbers 0, 0 and 1, 0 and 1 and 2, 0 and 1 and 2 and 3, ...
    // Obviously there is only one way we could form number 0
    // and it is with number 0 itself.
    for row in &mut partition_matrix {
        row[0] = 1.0;
    }

    // Now let's go through other possible options of how we could form number m out of
    // summands 0, 1, ..., m using Dynamic Programming approach.
    for summand_index in 1..=number {
        for number_index in 1..=number {
            partition_matrix[summand_index][number_index] = if summand_index > number_index {
                // If summand number is bigger then current number itself then just it won't add
                // any new ways of forming the number. Thus we may just copy the number from row above.
                partition_matrix[summand_index - 1][number_index]
            } else {
                // The number of combinations would equal to number of combinations of forming the same
                // number but WITHOUT current summand number PLUS number of combinations of forming the
                // <current number - current summand> number but WITH current summand.
                let combos_without_summand = partition_matrix[summand_index - 1][number_index];
                let combos_with_summand =
                    partition_matrix[summand_index][number_index - summand_index];
                combos_without_summand + combos_with_summand
            };
        }
    }

    partition_matrix[number][number]
}

js::export_default!(integer_partition);
