use crate::algorithms::math::matrix::{self as mtrx, Matrix};

/// The euclidean distance between two matrices of one shape, to two
/// decimal places.
pub fn euclidean_distance(a: &Matrix, b: &Matrix) -> f64 {
    mtrx::validate_same_shape(a, b);

    let mut squares_total = 0.0;

    mtrx::walk(a, |indices, a_cell_value| {
        let b_cell_value = mtrx::get_cell_at_index(b, indices);
        squares_total += (a_cell_value - b_cell_value).powi(2);
    });

    (squares_total.sqrt() * 100.0).round() / 100.0
}

js::export_default!(euclidean_distance);
