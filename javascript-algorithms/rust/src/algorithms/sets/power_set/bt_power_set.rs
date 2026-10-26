use js::Unknown;

/// Each subset of `original_set` from `start_at` on, each `current_sub_set`
/// grown by one, into `all_subsets`, by backtracking.
fn bt_power_set_recursive<'a>(
    original_set: &[&'a Unknown],
    all_subsets: &mut Vec<Vec<&'a Unknown>>,
    current_sub_set: &mut Vec<&'a Unknown>,
    start_at: usize,
) {
    // Let's iterate over originalSet elements that may be added to the subset
    // without having duplicates. The value of startAt prevents adding the duplicates.
    for position in start_at..original_set.len() {
        // Let's push current element to the subset
        current_sub_set.push(original_set[position]);

        // Current subset is already valid so let's memorize it.
        // We do array destruction here to save the clone of the currentSubSet.
        // We need to save a clone since the original currentSubSet is going to be
        // mutated in further recursive calls.
        all_subsets.push(current_sub_set.clone());

        // Let's try to generate all other subsets for the current subset.
        // We're increasing the position by one to avoid duplicates in subset.
        bt_power_set_recursive(original_set, all_subsets, current_sub_set, position + 1);

        // BACKTRACK. Exclude last element from the subset and try the next valid one.
        current_sub_set.pop();
    }
}

/// Find power-set of a set using BACKTRACKING approach.
pub fn bt_power_set<'a>(original_set: &[&'a Unknown]) -> Vec<Vec<&'a Unknown>> {
    let mut all_subsets = vec![Vec::new()];
    bt_power_set_recursive(original_set, &mut all_subsets, &mut Vec::new(), 0);
    all_subsets
}

js::export_default!(bt_power_set);
