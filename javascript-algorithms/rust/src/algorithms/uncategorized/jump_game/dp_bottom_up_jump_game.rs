/// DYNAMIC PROGRAMMING BOTTOM-UP approach of solving Jump Game.
pub fn dp_bottom_up_jump_game(numbers: &[usize]) -> bool {
    // Mark all cells as unknown except the last one, which is good: it's
    // the goal itself.
    let mut cells_goodness = vec![false; numbers.len()];
    cells_goodness[numbers.len() - 1] = true;

    // Go throw all cells starting from the one before the last
    // one (since the last one is "good" already) and fill cellsGoodness table.
    for cell_index in (0..numbers.len() - 1).rev() {
        let max_jump_length = numbers[cell_index].min(numbers.len() - 1 - cell_index);

        for jump_length in (1..=max_jump_length).rev() {
            let next_index = cell_index + jump_length;
            if cells_goodness[next_index] {
                cells_goodness[cell_index] = true;
                break;
            }
        }
    }

    // Now, if the zero's cell is good one then we can jump from it to the end of the array.
    cells_goodness[0]
}

js::export_default!(dp_bottom_up_jump_game);
