/// Brute Force solution: the run of `input_array` of the greatest sum.
/// Complexity: O(n^2)
pub fn bf_maximum_subarray(input_array: &[f64]) -> Vec<f64> {
    let mut max_subarray_start_index = 0;
    let mut max_subarray_length = 0;
    let mut max_subarray_sum = None;

    for start_index in 0..input_array.len() {
        let mut subarray_sum = 0.0;
        for arr_length in 1..=input_array.len() - start_index {
            subarray_sum += input_array[start_index + (arr_length - 1)];
            if max_subarray_sum.is_none_or(|max| subarray_sum > max) {
                max_subarray_sum = Some(subarray_sum);
                max_subarray_start_index = start_index;
                max_subarray_length = arr_length;
            }
        }
    }

    input_array[max_subarray_start_index..max_subarray_start_index + max_subarray_length].to_vec()
}

js::export_default!(bf_maximum_subarray);
