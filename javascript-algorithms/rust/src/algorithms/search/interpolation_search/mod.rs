/// Interpolation search implementation: where `seek_element` is in
/// `sorted_array`, guessing where it is by its value; -1 where it isn't.
pub fn interpolation_search(sorted_array: &[f64], seek_element: f64) -> isize {
    let mut left_index: isize = 0;
    let mut right_index = sorted_array.len() as isize - 1;

    while left_index <= right_index {
        let left = sorted_array[left_index as usize];
        let range_delta = sorted_array[right_index as usize] - left;
        let index_delta = (right_index - left_index) as f64;
        let value_delta = seek_element - left;

        // If valueDelta is less then zero it means that there is no seek element
        // exists in array since the lowest element from the range is already higher
        // then seek element.
        if value_delta < 0.0 {
            return -1;
        }

        // If range delta is zero then subarray contains all the same numbers
        // and thus there is nothing to search for unless this range is all
        // consists of seek number.
        if range_delta == 0.0 {
            // By doing this we're also avoiding division by zero while
            // calculating the middleIndex later.
            return if left == seek_element { left_index } else { -1 };
        }

        // Do interpolation of the middle index.
        let middle_index =
            left_index + ((value_delta * index_delta) / range_delta).floor() as isize;
        let middle = sorted_array[middle_index as usize];

        // If we've found the element just return its position.
        if middle == seek_element {
            return middle_index;
        }

        // Decide which half to choose for seeking next: left or right one.
        if middle < seek_element {
            // Go to the right half of the array.
            left_index = middle_index + 1;
        } else {
            // Go to the left half of the array.
            right_index = middle_index - 1;
        }
    }

    -1
}

js::export_default!(interpolation_search);
