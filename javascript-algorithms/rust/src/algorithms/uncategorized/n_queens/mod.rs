pub mod n_queens_bitwise;
pub mod queen_position;

use queen_position::QueenPosition;

fn is_safe(queens_positions: &[Option<QueenPosition>], row_index: i32, column_index: i32) -> bool {
    // New position to which the Queen is going to be placed.
    let new_queen_position = QueenPosition::new(row_index, column_index);

    // Check if new queen position conflicts with any other queens.
    for current_queen_position in queens_positions {
        if let Some(current_queen_position) = current_queen_position
            && (
                // Check if there are any queen on the same column.
                new_queen_position.column_index == current_queen_position.column_index
                // Check if there are any queen on the same row.
                || new_queen_position.row_index == current_queen_position.row_index
                // Check if there are any queen on the same left diagonal.
                || new_queen_position.left_diagonal() == current_queen_position.left_diagonal()
                // Check if there are any queen on the same right diagonal.
                || new_queen_position.right_diagonal() == current_queen_position.right_diagonal()
            )
        {
            // Can't place queen into current position since there are other queens that
            // are threatening it.
            return false;
        }
    }

    // Looks like we're safe.
    true
}

fn n_queens_recursive(
    solutions: &mut Vec<Vec<Option<QueenPosition>>>,
    previous_queens_positions: &[Option<QueenPosition>],
    queens_count: i32,
    row_index: i32,
) -> bool {
    // Clone positions array.
    let mut queens_positions: Vec<Option<QueenPosition>> = previous_queens_positions
        .iter()
        .map(|queen_position| {
            queen_position.as_ref().map(|queen_position| {
                QueenPosition::new(queen_position.row_index, queen_position.column_index)
            })
        })
        .collect();

    if row_index == queens_count {
        // We've successfully reached the end of the board.
        // Store solution to the list of solutions.
        solutions.push(queens_positions);

        // Solution found.
        return true;
    }

    // Let's try to put queen at row rowIndex into its safe column position.
    for column_index in 0..queens_count {
        if is_safe(&queens_positions, row_index, column_index) {
            // Place current queen to its current position.
            queens_positions[row_index as usize] =
                Some(QueenPosition::new(row_index, column_index));

            // Try to place all other queens as well.
            n_queens_recursive(solutions, &queens_positions, queens_count, row_index + 1);

            // BACKTRACKING.
            // Remove the queen from the row to avoid isSafe() returning false.
            queens_positions[row_index as usize] = None;
        }
    }

    false
}

pub fn n_queens(queens_count: i32) -> Vec<Vec<Option<QueenPosition>>> {
    // This array will hold positions or coordinates of each of
    // N queens in form of [rowIndex, columnIndex].
    let queens_positions: Vec<Option<QueenPosition>> = (0..queens_count).map(|_| None).collect();

    let mut solutions = Vec::new();

    // Solve problem recursively.
    n_queens_recursive(&mut solutions, &queens_positions, queens_count, 0);

    solutions
}

js::export_default!(n_queens);
