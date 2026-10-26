fn get_possible_moves(chessboard: &[Vec<bool>], position: [isize; 2]) -> Vec<[isize; 2]> {
    // Generate all knight moves (even those that go beyond the board).
    let possible_moves = [
        [position[0] - 1, position[1] - 2],
        [position[0] - 2, position[1] - 1],
        [position[0] + 1, position[1] - 2],
        [position[0] + 2, position[1] - 1],
        [position[0] - 2, position[1] + 1],
        [position[0] - 1, position[1] + 2],
        [position[0] + 1, position[1] + 2],
        [position[0] + 2, position[1] + 1],
    ];

    // Filter out all moves that go beyond the board.
    possible_moves
        .into_iter()
        .filter(|&[x, y]| {
            let board_size = chessboard.len() as isize;
            x >= 0 && y >= 0 && x < board_size && y < board_size
        })
        .collect()
}

fn is_move_allowed(chessboard: &[Vec<bool>], [x, y]: [isize; 2]) -> bool {
    !chessboard[x as usize][y as usize]
}

fn is_board_completely_visited(chessboard: &[Vec<bool>], moves: &[[isize; 2]]) -> bool {
    let total_possible_moves_count = chessboard.len().pow(2);
    let existing_moves_count = moves.len();

    total_possible_moves_count == existing_moves_count
}

fn knight_tour_recursive(chessboard: &mut [Vec<bool>], moves: &mut Vec<[isize; 2]>) -> bool {
    // If board has been completely visited then we've found a solution.
    if is_board_completely_visited(chessboard, moves) {
        return true;
    }

    // Get next possible knight moves.
    let last_move = moves[moves.len() - 1];
    let possible_moves = get_possible_moves(chessboard, last_move);

    // Try to do next possible moves.
    for current_move in possible_moves {
        // Check if current move is allowed. We aren't allowed to go to
        // the same cells twice.
        if is_move_allowed(chessboard, current_move) {
            // Actually do the move.
            moves.push(current_move);
            chessboard[current_move[0] as usize][current_move[1] as usize] = true;

            // If further moves starting from current are successful then
            // return true meaning the solution is found.
            if knight_tour_recursive(chessboard, moves) {
                return true;
            }

            // BACKTRACKING.
            // If current move was unsuccessful then step back and try to do another move.
            moves.pop();
            chessboard[current_move[0] as usize][current_move[1] as usize] = false;
        }
    }

    // Return false if we haven't found solution.
    false
}

pub fn knight_tour(chessboard_size: usize) -> Vec<[isize; 2]> {
    // Init chessboard.
    let mut chessboard = vec![vec![false; chessboard_size]; chessboard_size];

    // Init moves array.
    let mut moves = Vec::new();

    // Do first move and place the knight to the 0x0 cell.
    let first_move = [0, 0];
    moves.push(first_move);
    chessboard[first_move[0] as usize][first_move[0] as usize] = true;

    // Recursively try to do the next move.
    let solution_was_found = knight_tour_recursive(&mut chessboard, &mut moves);

    if solution_was_found {
        moves
    } else {
        Vec::new()
    }
}

js::export_default!(knight_tour);
