/// Divide and Conquer solution: the greatest sum of a run of
/// `input_array`, -Infinity of none.
/// Complexity: O(n^2) in case if no memoization applied
pub fn dc_maximum_subarray_sum(input_array: &[f64]) -> f64 {
    /// We are going through the inputArray array and for each element we have two options:
    /// - to pick
    /// - not to pick
    ///
    /// Also keep in mind, that the maximum sub-array must be contiguous. It means if we picked
    /// the element, we need to continue picking the next elements or stop counting the max sum.
    fn solve_recursively(input_array: &[f64], element_index: usize, must_pick: bool) -> f64 {
        if element_index >= input_array.len() {
            return if must_pick { 0.0 } else { f64::NEG_INFINITY };
        }
        (input_array[element_index] + solve_recursively(input_array, element_index + 1, true)).max(
            if must_pick {
                0.0
            } else {
                solve_recursively(input_array, element_index + 1, false)
            },
        )
    }

    solve_recursively(input_array, 0, false)
}

js::export_default!(dc_maximum_subarray_sum);
