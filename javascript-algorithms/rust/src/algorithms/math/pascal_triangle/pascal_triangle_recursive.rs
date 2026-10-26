/// The coefficients of Pascal's triangle's line `line_number`, from 0, each
/// the sum of the two above it in the line before.
pub fn pascal_triangle_recursive(line_number: usize) -> Vec<f64> {
    if line_number == 0 {
        return vec![1.0];
    }
    let current_line_size = line_number + 1;
    let previous_line_size = current_line_size - 1;
    // Create container for current line values.
    let mut current_line = Vec::with_capacity(current_line_size);
    // We'll calculate current line based on previous one.
    let previous_line = pascal_triangle_recursive(line_number - 1);
    // Let's go through all elements of current line except the first and
    // last one (since they were and will be filled with 1's) and calculate
    // current coefficient based on previous line.
    for num_index in 0..current_line_size {
        let left_coefficient = if num_index > 0 {
            previous_line[num_index - 1]
        } else {
            0.0
        };
        let right_coefficient = if num_index < previous_line_size {
            previous_line[num_index]
        } else {
            0.0
        };
        current_line.push(left_coefficient + right_coefficient);
    }
    current_line
}

js::export_default!(pascal_triangle_recursive);
