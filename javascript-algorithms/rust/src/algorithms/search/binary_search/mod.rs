use crate::utils::comparator::{Comparator, CompareFunction};
use js::Unknown;

/// Binary search implementation: where `seek_element` is in
/// `sorted_array`, halving the part it may be in each step; -1 where it
/// isn't.
pub fn binary_search(
    sorted_array: &[&Unknown],
    seek_element: &Unknown,
    comparator_callback: Option<CompareFunction>,
) -> isize {
    // Let's create comparator from the comparatorCallback function.
    // Comparator object will give us common comparison methods like equal() and lessThan().
    let comparator = Comparator::new(comparator_callback);

    // These two indices will contain current array (sub-array) boundaries.
    let mut start_index: isize = 0;
    let mut end_index = sorted_array.len() as isize - 1;

    // Let's continue to split array until boundaries are collapsed
    // and there is nothing to split anymore.
    while start_index <= end_index {
        // Let's calculate the index of the middle element.
        let middle_index = start_index + (end_index - start_index) / 2;
        let middle = sorted_array[middle_index as usize];

        // If we've found the element just return its position.
        if comparator.equal(middle, seek_element) {
            return middle_index;
        }

        // Decide which half to choose for seeking next: left or right one.
        if comparator.less_than(middle, seek_element) {
            // Go to the right half of the array.
            start_index = middle_index + 1;
        } else {
            // Go to the left half of the array.
            end_index = middle_index - 1;
        }
    }

    // Return -1 if we have not found anything.
    -1
}

js::export_default!(binary_search);
