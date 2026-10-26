/// The number of unique paths from the last of `steps` already made to a
/// `width` by `height` board's bottom right, `unique_steps` found so far.
fn paths_from(width: i32, height: i32, steps: &mut Vec<[i32; 2]>, unique_steps: f64) -> f64 {
    // Fetch current position on board.
    let current_pos = steps[steps.len() - 1];

    // Check if we've reached the end.
    if current_pos[0] == width - 1 && current_pos[1] == height - 1 {
        // In case if we've reached the end let's increase total
        // number of unique steps.
        return unique_steps + 1.0;
    }

    // Let's calculate how many unique path we will have
    // by going right and by going down.
    let mut right_unique_steps = 0.0;
    let mut down_unique_steps = 0.0;

    // Do right step if possible.
    if current_pos[0] < width - 1 {
        steps.push([current_pos[0] + 1, current_pos[1]]);

        // Calculate how many unique paths we'll get by moving right.
        right_unique_steps = paths_from(width, height, steps, unique_steps);

        // BACKTRACK and try another move.
        steps.pop();
    }

    // Do down step if possible.
    if current_pos[1] < height - 1 {
        steps.push([current_pos[0], current_pos[1] + 1]);

        // Calculate how many unique paths we'll get by moving down.
        down_unique_steps = paths_from(width, height, steps, unique_steps);

        // BACKTRACK and try another move.
        steps.pop();
    }

    // Total amount of unique steps will be equal to total amount of
    // unique steps by going right plus total amount of unique steps
    // by going down.
    right_unique_steps + down_unique_steps
}

/// BACKTRACKING approach of solving Unique Paths problem.
pub fn bt_unique_paths(width: i32, height: i32) -> f64 {
    paths_from(width, height, &mut vec![[0, 0]], 0.0)
}

js::export_default!(bt_unique_paths);
