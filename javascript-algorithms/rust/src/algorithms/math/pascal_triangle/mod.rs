pub mod pascal_triangle_recursive;

/// The coefficients of Pascal's triangle's line `line_number`, from 0.
pub fn pascal_triangle(line_number: usize) -> Vec<f64> {
    let mut current_line = vec![1.0];
    let current_line_size = line_number + 1;
    for num_index in 1..current_line_size {
        // See explanation of this formula in README.
        let coefficient =
            current_line[num_index - 1] * (line_number - num_index + 1) as f64 / num_index as f64;
        current_line.push(coefficient);
    }
    current_line
}

js::export_default!(pascal_triangle);
