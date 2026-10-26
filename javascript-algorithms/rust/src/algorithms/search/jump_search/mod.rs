use crate::utils::comparator::{Comparator, CompareFunction};
use js::Unknown;

/// Jump (block) search implementation: where `seek_element` is in
/// `sorted_array`, jumping a block at a time, then stepping through the
/// block it may be in; -1 where it isn't.
pub fn jump_search(
    sorted_array: &[&Unknown],
    seek_element: &Unknown,
    comparator_callback: Option<CompareFunction>,
) -> isize {
    let comparator = Comparator::new(comparator_callback);
    let array_size = sorted_array.len();

    if array_size == 0 {
        // We can't find anything in empty array.
        return -1;
    }

    // Calculate optimal jump size.
    // Total number of comparisons in the worst case will be ((arraySize/jumpSize) + jumpSize - 1).
    // The value of the function ((arraySize/jumpSize) + jumpSize - 1) will be minimum
    // when jumpSize = √array.length.
    let jump_size = array_size.isqrt();

    // Find the block where the seekElement belong to.
    let mut block_start = 0;
    let mut block_end = jump_size;
    while comparator.greater_than(seek_element, sorted_array[block_end.min(array_size) - 1]) {
        // Jump to the next block.
        block_start = block_end;
        block_end += jump_size;

        // If our next block is out of array then we couldn't found the element.
        if block_start > array_size {
            return -1;
        }
    }

    // Do linear search for seekElement in subarray starting from blockStart.
    let mut current_index = block_start;
    while current_index < block_end.min(array_size) {
        if comparator.equal(sorted_array[current_index], seek_element) {
            return current_index as isize;
        }

        current_index += 1;
    }

    -1
}

js::export_default!(jump_search);
