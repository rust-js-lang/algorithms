use js::Unknown;

/// Each order of `permutation_options`, each option once.
pub fn permutate_without_repetitions<'a>(
    permutation_options: &[&'a Unknown],
) -> Vec<Vec<&'a Unknown>> {
    if permutation_options.len() == 1 {
        return vec![permutation_options.to_vec()];
    }

    // Init permutations array.
    let mut permutations = Vec::new();

    // Get all permutations for permutationOptions excluding the first element.
    let smaller_permutations = permutate_without_repetitions(&permutation_options[1..]);

    // Insert first option into every possible position of every smaller permutation.
    let first_option = permutation_options[0];

    for smaller_permutation in &smaller_permutations {
        for position_index in 0..=smaller_permutation.len() {
            let permutation_prefix = &smaller_permutation[..position_index];
            let permutation_suffix = &smaller_permutation[position_index..];
            permutations.push([permutation_prefix, &[first_option], permutation_suffix].concat());
        }
    }

    permutations
}

js::export_default!(permutate_without_repetitions);
