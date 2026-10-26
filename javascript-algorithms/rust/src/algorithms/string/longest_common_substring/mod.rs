/// The longest run of characters both strings have.
pub fn longest_common_substring(string1: &str, string2: &str) -> String {
    // Convert strings to arrays to treat unicode symbols length correctly.
    // For example:
    // '𐌵'.length === 2
    // [...'𐌵'].length === 1
    let s1: Vec<char> = string1.chars().collect();
    let s2: Vec<char> = string2.chars().collect();

    // Init the matrix of all substring lengths to use Dynamic Programming approach.
    // Its first row and column, of an empty string, are zeros.
    let mut substring_matrix = vec![vec![0; s1.len() + 1]; s2.len() + 1];

    // Build the matrix of all substring lengths to use Dynamic Programming approach.
    let mut longest_substring_length = 0;
    let mut longest_substring_column = 0;
    let mut longest_substring_row = 0;

    for row_index in 1..=s2.len() {
        for column_index in 1..=s1.len() {
            substring_matrix[row_index][column_index] = if s1[column_index - 1] == s2[row_index - 1]
            {
                substring_matrix[row_index - 1][column_index - 1] + 1
            } else {
                0
            };

            // Try to find the biggest length of all common substring lengths
            // and to memorize its last character position (indices)
            if substring_matrix[row_index][column_index] > longest_substring_length {
                longest_substring_length = substring_matrix[row_index][column_index];
                longest_substring_column = column_index;
                longest_substring_row = row_index;
            }
        }
    }

    if longest_substring_length == 0 {
        // Longest common substring has not been found.
        return String::new();
    }

    // Detect the longest substring from the matrix.
    let mut longest_substring = String::new();

    while substring_matrix[longest_substring_row][longest_substring_column] > 0 {
        longest_substring.insert(0, s1[longest_substring_column - 1]);
        longest_substring_row -= 1;
        longest_substring_column -= 1;
    }

    longest_substring
}

js::export_default!(longest_common_substring);
