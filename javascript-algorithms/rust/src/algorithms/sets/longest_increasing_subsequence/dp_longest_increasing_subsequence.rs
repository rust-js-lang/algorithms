/// Dynamic programming approach to find longest increasing subsequence.
/// Complexity: O(n * n)
pub fn dp_longest_increasing_subsequence(sequence: &[f64]) -> usize {
    // Create array with longest increasing substrings length and
    // fill it with 1-s that would mean that each element of the sequence
    // is itself a minimum increasing subsequence.
    let mut lengths_array = vec![1; sequence.len()];

    let mut previous_element_index = 0;
    let mut current_element_index = 1;

    while current_element_index < sequence.len() {
        if sequence[previous_element_index] < sequence[current_element_index] {
            // If current element is bigger then the previous one then
            // current element is a part of increasing subsequence which
            // length is by one bigger then the length of increasing subsequence
            // for previous element.
            let new_length = lengths_array[previous_element_index] + 1;
            if new_length > lengths_array[current_element_index] {
                // Increase only if previous element would give us bigger subsequence length
                // then we already have for current element.
                lengths_array[current_element_index] = new_length;
            }
        }

        // Move previous element index right.
        previous_element_index += 1;

        // If previous element index equals to current element index then
        // shift current element right and reset previous element index to zero.
        if previous_element_index == current_element_index {
            current_element_index += 1;
            previous_element_index = 0;
        }
    }

    // Find the biggest element in lengthsArray.
    // This number is the biggest length of increasing subsequence.
    lengths_array.into_iter().max().unwrap_or(0)
}

js::export_default!(dp_longest_increasing_subsequence);
