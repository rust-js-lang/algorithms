/// GREEDY approach of solving Jump Game: the leftmost cell known to reach
/// the end, moved left while each cell can jump to it.
pub fn greedy_jump_game(numbers: &[usize]) -> bool {
    // The "good" cell is a cell from which we may jump to the last cell of the numbers array.

    // The last cell in numbers array is for sure the "good" one since it is our goal to reach.
    let mut left_good_position = numbers.len() - 1;

    // Go through all numbers from right to left.
    for number_index in (0..numbers.len() - 1).rev() {
        // If we can reach the "good" cell from the current one then for sure the current
        // one is also "good". Since after all we'll be able to reach the end of the array
        // from it.
        let max_current_jump_length = number_index + numbers[number_index];
        if max_current_jump_length >= left_good_position {
            left_good_position = number_index;
        }
    }

    // If the most left "good" position is the zero's one then we may say that it IS
    // possible jump to the end of the array from the first cell;
    left_good_position == 0
}

js::export_default!(greedy_jump_game);
