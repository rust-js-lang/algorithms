use js::Unknown;

/// Each sequence of `permutation_length` of `permutation_options`, as
/// many as there are options where it's left out, an option any number of
/// times.
pub fn permutate_with_repetitions<'a>(
    permutation_options: &[&'a Unknown],
    permutation_length: Option<usize>,
) -> Vec<Vec<&'a Unknown>> {
    let permutation_length = permutation_length.unwrap_or(permutation_options.len());
    if permutation_length == 1 {
        return permutation_options
            .iter()
            .map(|&permutation_option| vec![permutation_option])
            .collect();
    }

    // Init permutations array.
    let mut permutations = Vec::new();

    // Get smaller permutations.
    let smaller_permutations =
        permutate_with_repetitions(permutation_options, Some(permutation_length - 1));

    // Go through all options and join it to the smaller permutations.
    for &current_option in permutation_options {
        for smaller_permutation in &smaller_permutations {
            permutations.push([&[current_option], smaller_permutation.as_slice()].concat());
        }
    }

    permutations
}

js::export_default!(permutate_with_repetitions);
