const ZERO_OR_MORE_CHARS: char = '*';
const ANY_CHAR: char = '.';

/// Whether `pattern`, of `.` for any character and `*` for any number of
/// the one before, matches the whole of `string`, by Dynamic Programming.
pub fn regular_expression_matching(string: &str, pattern: &str) -> bool {
    let string: Vec<char> = string.chars().collect();
    let pattern: Vec<char> = pattern.chars().collect();

    // Let's initiate dynamic programming matrix for this string and pattern.
    // We will have pattern characters on top (as columns) and string characters
    // will be placed to the left of the table (as rows).
    let mut match_matrix = vec![vec![false; pattern.len() + 1]; string.len() + 1];

    // Address empty pattern and empty string case.
    match_matrix[0][0] = true;

    // Deal with first row of the matrix, which is all `false` but where a
    // `*` repeats what's before it none, `a*` of an empty string.
    for column_index in 1..=pattern.len() {
        let pattern_index = column_index - 1;

        if pattern[pattern_index] == ZERO_OR_MORE_CHARS {
            match_matrix[0][column_index] = match_matrix[0][column_index - 2];
        }
    }

    for row_index in 1..=string.len() {
        for column_index in 1..=pattern.len() {
            let string_index = row_index - 1;
            let pattern_index = column_index - 1;

            match_matrix[row_index][column_index] = if pattern[pattern_index] == ZERO_OR_MORE_CHARS
            {
                // In case if current pattern character is special '*' character we have
                // two options: the character before it is repeated none, or once more.
                match_matrix[row_index][column_index - 2]
                    || ((pattern[pattern_index - 1] == string[string_index]
                        || pattern[pattern_index - 1] == ANY_CHAR)
                        && match_matrix[row_index - 1][column_index])
            } else if pattern[pattern_index] == string[string_index]
                || pattern[pattern_index] == ANY_CHAR
            {
                // In case if current pattern char is the same as current string char
                // or it may be any character (in case if pattern contains '.' char)
                // we need to check if there was a match for the pattern and for the
                // string by WITHOUT current char.
                match_matrix[row_index - 1][column_index - 1]
            } else {
                // If current pattern char and current string char are different
                // then their match is false.
                false
            };
        }
    }

    match_matrix[string.len()][pattern.len()]
}

js::export_default!(regular_expression_matching);
