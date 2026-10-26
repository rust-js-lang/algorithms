/// The matrix turned a quarter clockwise, its rows' items swapped in place.
pub fn square_matrix_rotation<T>(mut matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    // Do top-right/bottom-left diagonal reflection of the matrix.
    for row_index in 0..matrix.len() {
        for column_index in row_index + 1..matrix.len() {
            // Swap elements.
            let (upper, lower) = matrix.split_at_mut(column_index);
            std::mem::swap(
                &mut lower[0][row_index],
                &mut upper[row_index][column_index],
            );
        }
    }

    // Do horizontal reflection of the matrix.
    let size = matrix.len();
    for row in &mut matrix {
        for column_index in 0..size / 2 {
            // Swap elements.
            row.swap(size - column_index - 1, column_index);
        }
    }

    matrix
}

js::export_default!(square_matrix_rotation);
