/// Dynamic Programming solution: the run of `input_array` of the greatest
/// sum, by Kadane's algorithm.
/// Complexity: O(n)
pub fn dp_maximum_subarray(input_array: &[f64]) -> Vec<f64> {
    // We iterate through the inputArray once, using a greedy approach to keep track of the maximum
    // sum we've seen so far and the current sum.
    //
    // The currentSum variable gets reset to 0 every time it drops below 0.
    //
    // The maxSum variable is set to -Infinity so that if all numbers are negative, the highest
    // negative number will constitute the maximum subarray.

    let mut max_sum = f64::NEG_INFINITY;
    let mut current_sum = 0.0;

    // We need to keep track of the starting and ending indices that contributed to our maxSum
    // so that we can return the actual subarray. From the beginning let's assume that whole array
    // is contributing to maxSum.
    let mut max_start_index = 0;
    let mut max_end_index = input_array.len();
    let mut current_start_index = 0;

    for (current_index, &current_number) in input_array.iter().enumerate() {
        current_sum += current_number;

        // Update maxSum and the corresponding indices if we have found a new max.
        if max_sum < current_sum {
            max_sum = current_sum;
            max_start_index = current_start_index;
            max_end_index = current_index + 1;
        }

        // Reset currentSum and currentStartIndex if currentSum drops below 0.
        if current_sum < 0.0 {
            current_sum = 0.0;
            current_start_index = current_index + 1;
        }
    }

    input_array[max_start_index..max_end_index].to_vec()
}

js::export_default!(dp_maximum_subarray);
