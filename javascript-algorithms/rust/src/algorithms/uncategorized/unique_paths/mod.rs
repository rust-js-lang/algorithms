pub mod bt_unique_paths;
pub mod dp_unique_paths;

use crate::algorithms::math::pascal_triangle::pascal_triangle;

/// The number of unique paths from a `width` by `height` board's top left
/// to its bottom right, by Pascal's triangle.
pub fn unique_paths(width: usize, height: usize) -> f64 {
    let pascal_line = width + height - 2;
    let pascal_line_position = width.min(height) - 1;

    pascal_triangle(pascal_line)[pascal_line_position]
}

js::export_default!(unique_paths);
