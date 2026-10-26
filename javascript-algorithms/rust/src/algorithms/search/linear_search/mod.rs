use crate::utils::comparator::{Comparator, CompareFunction};
use js::Unknown;

/// Linear search implementation: each index `seek_element` is at in
/// `array`.
pub fn linear_search(
    array: &[&Unknown],
    seek_element: &Unknown,
    comparator_callback: Option<CompareFunction>,
) -> Vec<usize> {
    let comparator = Comparator::new(comparator_callback);
    let mut found_indices = Vec::new();

    for (index, element) in array.iter().enumerate() {
        if comparator.equal(element, seek_element) {
            found_indices.push(index);
        }
    }

    found_indices
}

js::export_default!(linear_search);
