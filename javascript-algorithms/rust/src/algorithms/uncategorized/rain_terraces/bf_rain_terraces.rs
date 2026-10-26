/// BRUTE FORCE approach of solving Trapping Rain Water problem.
pub fn bf_rain_terraces(terraces: &[f64]) -> f64 {
    let mut water_amount = 0.0;

    for (terrace_index, &terrace) in terraces.iter().enumerate() {
        // Get left most high terrace.
        let mut left_highest_level: f64 = 0.0;
        for &left in terraces[..terrace_index].iter().rev() {
            left_highest_level = left_highest_level.max(left);
        }

        // Get right most high terrace.
        let mut right_highest_level: f64 = 0.0;
        for &right in &terraces[terrace_index + 1..] {
            right_highest_level = right_highest_level.max(right);
        }

        // Add current terrace water amount.
        let terrace_boundary_level = left_highest_level.min(right_highest_level);
        if terrace_boundary_level > terrace {
            // Terrace will be able to store the water if the lowest of two left and right highest
            // terraces are still higher than the current one.
            water_amount += terrace_boundary_level - terrace;
        }
    }

    water_amount
}

js::export_default!(bf_rain_terraces);
