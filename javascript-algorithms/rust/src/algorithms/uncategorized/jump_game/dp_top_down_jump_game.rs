/// Whether the last of `numbers` is reached from `start_index`, each cell
/// already found unable to reach it, `cells_goodness`'s `false`, skipped.
fn jumps_from(
    numbers: &[usize],
    start_index: usize,
    current_jumps: &mut Vec<usize>,
    cells_goodness: &[Option<bool>],
) -> bool {
    if start_index == numbers.len() - 1 {
        // We've jumped directly to last cell. This situation is a solution.
        return true;
    }

    // Init cell goodness table if it is empty.
    // This is DYNAMIC PROGRAMMING feature.
    let mut current_cells_goodness = cells_goodness.to_vec();
    if current_cells_goodness.is_empty() {
        current_cells_goodness = vec![None; numbers.len()];
    }

    // Check what jumps are possible from current position.
    // We don't need to jump beyond the array.
    let max_jump_length = numbers[start_index].min(numbers.len() - 1 - start_index);

    // Let's start jumping from startIndex and see whether any
    // jump is successful and has reached the end of the array.
    for jump_length in (1..=max_jump_length).rev() {
        // Try next jump.
        let next_index = start_index + jump_length;

        // Jump only into "good" or "unknown" cells.
        // This is top-down dynamic programming optimisation of backtracking algorithm.
        if current_cells_goodness[next_index] != Some(false) {
            current_jumps.push(next_index);

            let is_jump_successful =
                jumps_from(numbers, next_index, current_jumps, &current_cells_goodness);

            // Check if current jump was successful.
            if is_jump_successful {
                return true;
            }

            // BACKTRACKING!
            // If previous jump wasn't successful then retreat and try the next one.
            current_jumps.pop();

            // Mark current cell as "bad" to avoid its deep visiting later.
            current_cells_goodness[next_index] = Some(false);
        }
    }

    false
}

/// DYNAMIC PROGRAMMING TOP-DOWN approach of solving Jump Game.
pub fn dp_top_down_jump_game(numbers: &[usize]) -> bool {
    jumps_from(numbers, 0, &mut Vec::new(), &[])
}

js::export_default!(dp_top_down_jump_game);
