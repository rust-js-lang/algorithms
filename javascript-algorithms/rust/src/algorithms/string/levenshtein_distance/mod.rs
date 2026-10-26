/// The fewest insertions, deletions and substitutions that make `a` `b`.
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    // Create empty edit distance matrix for all possible modifications of
    // substrings of a to substrings of b.
    let mut distance_matrix = vec![vec![0; a.len() + 1]; b.len() + 1];

    // Fill the first row of the matrix.
    // If this is first row then we're transforming empty string to a.
    // In this case the number of transformations equals to size of a substring.
    for (i, cell) in distance_matrix[0].iter_mut().enumerate() {
        *cell = i;
    }

    // Fill the first column of the matrix.
    // If this is first column then we're transforming empty string to b.
    // In this case the number of transformations equals to size of b substring.
    for (j, row) in distance_matrix.iter_mut().enumerate() {
        row[0] = j;
    }

    for j in 1..=b.len() {
        for i in 1..=a.len() {
            let indicator = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            distance_matrix[j][i] = (distance_matrix[j][i - 1] + 1) // deletion
                .min(distance_matrix[j - 1][i] + 1) // insertion
                .min(distance_matrix[j - 1][i - 1] + indicator); // substitution
        }
    }

    distance_matrix[b.len()][a.len()]
}

js::export_default!(levenshtein_distance);
