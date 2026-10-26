/// Whether the last of `numbers` is reached from `start_index`, each a
/// jump of at most its number, `current_jumps` the jumps made, trying the
/// longest first and backtracking.
fn jumps_from(numbers: &[usize], start_index: usize, current_jumps: &mut Vec<usize>) -> bool {
    if start_index == numbers.len() - 1 {
        // We've jumped directly to last cell. This situation is a solution.
        return true;
    }

    // Check what jumps are possible from current position.
    // We don't need to jump beyond the array.
    let max_jump_length = numbers[start_index].min(numbers.len() - 1 - start_index);

    // Let's start jumping from startIndex and see whether any
    // jump is successful and has reached the end of the array.
    for jump_length in (1..=max_jump_length).rev() {
        // Try next jump.
        let next_index = start_index + jump_length;
        current_jumps.push(next_index);

        let is_jump_successful = jumps_from(numbers, next_index, current_jumps);

        // Check if current jump was successful.
        if is_jump_successful {
            return true;
        }

        // BACKTRACKING!
        // If previous jump wasn't successful then retreat and try the next one.
        current_jumps.pop();
    }

    false
}

/// BACKTRACKING approach of solving Jump Game.
pub fn backtracking_jump_game(numbers: &[usize]) -> bool {
    jumps_from(numbers, 0, &mut Vec::new())
}

js::export_default!(backtracking_jump_game);
