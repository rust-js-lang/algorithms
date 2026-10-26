/// DYNAMIC PROGRAMMING approach of solving Unique Paths problem.
pub fn dp_unique_paths(width: usize, height: usize) -> f64 {
    // Init board.
    let mut board = vec![vec![0.0; width]; height];

    // Base case.
    // There is only one way of getting to board[0][any] and
    // there is also only one way of getting to board[any][0].
    // This is because we have a restriction of moving right
    // and down only.
    for (row_index, row) in board.iter_mut().enumerate() {
        for (column_index, cell) in row.iter_mut().enumerate() {
            if row_index == 0 || column_index == 0 {
                *cell = 1.0;
            }
        }
    }

    // Now, since we have this restriction of moving only to the right
    // and down we might say that number of unique paths to the current
    // cell is a sum of numbers of unique paths to the cell above the
    // current one and to the cell to the left of current one.
    for row_index in 1..height {
        for column_index in 1..width {
            let uniques_from_top = board[row_index - 1][column_index];
            let uniques_from_left = board[row_index][column_index - 1];
            board[row_index][column_index] = uniques_from_top + uniques_from_left;
        }
    }

    board[height - 1][width - 1]
}

js::export_default!(dp_unique_paths);
