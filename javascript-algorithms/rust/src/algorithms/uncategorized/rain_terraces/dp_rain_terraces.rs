/// DYNAMIC PROGRAMMING approach of solving Trapping Rain Water problem.
pub fn dp_rain_terraces(terraces: &[f64]) -> f64 {
    let mut water_amount = 0.0;

    // Init arrays that will keep the list of left and right maximum levels for specific positions.
    let mut left_max_levels = vec![0.0; terraces.len()];
    let mut right_max_levels = vec![0.0; terraces.len()];
    let (Some(&first), Some(&last)) = (terraces.first(), terraces.last()) else {
        return water_amount;
    };

    // Calculate the highest terrace level from the LEFT relative to the current terrace.
    left_max_levels[0] = first;
    for terrace_index in 1..terraces.len() {
        left_max_levels[terrace_index] =
            terraces[terrace_index].max(left_max_levels[terrace_index - 1]);
    }

    // Calculate the highest terrace level from the RIGHT relative to the current terrace.
    right_max_levels[terraces.len() - 1] = last;
    for terrace_index in (0..terraces.len() - 1).rev() {
        right_max_levels[terrace_index] =
            terraces[terrace_index].max(right_max_levels[terrace_index + 1]);
    }

    // Not let's go through all terraces one by one and calculate how much water
    // each terrace may accumulate based on previously calculated values.
    for (terrace_index, &terrace) in terraces.iter().enumerate() {
        // Pick the lowest from the left/right highest terraces.
        let current_terrace_boundary =
            left_max_levels[terrace_index].min(right_max_levels[terrace_index]);

        if current_terrace_boundary > terrace {
            water_amount += current_terrace_boundary - terrace;
        }
    }

    water_amount
}

js::export_default!(dp_rain_terraces);
