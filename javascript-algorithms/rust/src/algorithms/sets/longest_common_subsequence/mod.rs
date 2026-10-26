pub mod longest_common_subsequence_recursive;

/// The longest sequence of items both sets have, in their order; `[""]`
/// where they share none.
pub fn longest_common_subsequence(set1: &[&str], set2: &[&str]) -> Vec<String> {
    // Init LCS matrix. Its first row and column, of an empty set, are zeros.
    let mut lcs_matrix = vec![vec![0; set1.len() + 1]; set2.len() + 1];

    for row_index in 1..=set2.len() {
        for column_index in 1..=set1.len() {
            lcs_matrix[row_index][column_index] = if set1[column_index - 1] == set2[row_index - 1] {
                lcs_matrix[row_index - 1][column_index - 1] + 1
            } else {
                lcs_matrix[row_index - 1][column_index].max(lcs_matrix[row_index][column_index - 1])
            };
        }
    }

    // Calculate LCS based on LCS matrix.
    if lcs_matrix[set2.len()][set1.len()] == 0 {
        // If the length of largest common string is zero then return empty string.
        return vec![String::new()];
    }

    let mut longest_sequence = Vec::new();
    let mut column_index = set1.len();
    let mut row_index = set2.len();

    while column_index > 0 || row_index > 0 {
        if column_index > 0 && row_index > 0 && set1[column_index - 1] == set2[row_index - 1] {
            // Move by diagonal left-top.
            longest_sequence.insert(0, set1[column_index - 1].to_string());
            column_index -= 1;
            row_index -= 1;
        } else if column_index > 0
            && lcs_matrix[row_index][column_index] == lcs_matrix[row_index][column_index - 1]
        {
            // Move left.
            column_index -= 1;
        } else {
            // Move up.
            row_index -= 1;
        }
    }

    longest_sequence
}

js::export_default!(longest_common_subsequence);
