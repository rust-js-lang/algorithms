/// Each combination of `candidates`, each any number of times, summing to
/// `remaining_sum`, into `final_combinations`.
fn combination_sum_recursive(
    candidates: &[f64],
    remaining_sum: f64,
    final_combinations: &mut Vec<Vec<f64>>,
    current_combination: &mut Vec<f64>,
    start_from: usize,
) {
    if remaining_sum < 0.0 {
        // By adding another candidate we've gone below zero.
        // This would mean that the last candidate was not acceptable.
        return;
    }

    if remaining_sum == 0.0 {
        // If after adding the previous candidate our remaining sum
        // became zero - we need to save the current combination since it is one
        // of the answers we're looking for.
        final_combinations.push(current_combination.clone());
        return;
    }

    // If we haven't reached zero yet let's continue to add all
    // possible candidates that are left.
    for candidate_index in start_from..candidates.len() {
        let current_candidate = candidates[candidate_index];

        // Let's try to add another candidate.
        current_combination.push(current_candidate);

        // Explore further option with current candidate being added.
        combination_sum_recursive(
            candidates,
            remaining_sum - current_candidate,
            final_combinations,
            current_combination,
            candidate_index,
        );

        // BACKTRACKING.
        // Let's get back, exclude current candidate and try another ones later.
        current_combination.pop();
    }
}

/// Backtracking algorithm of finding all possible combination for specific sum.
pub fn combination_sum(candidates: &[f64], target: f64) -> Vec<Vec<f64>> {
    let mut final_combinations = Vec::new();
    combination_sum_recursive(
        candidates,
        target,
        &mut final_combinations,
        &mut Vec::new(),
        0,
    );
    final_combinations
}

js::export_default!(combination_sum);
